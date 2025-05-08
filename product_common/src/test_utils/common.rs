// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

use super::utils::request_funds;
use crate::core_client::{CoreClient, CoreClientReadOnly};
#[cfg(feature = "transaction")]
use crate::transaction::transaction_builder::Transaction;
#[cfg(feature = "transaction")]
use crate::transaction::transaction_builder::TransactionBuilder;
use crate::Error;
use anyhow::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use iota_interaction::ident_str;
use iota_interaction::keytool_signer::KeytoolSigner;
use iota_interaction::move_types::language_storage::StructTag;
use iota_interaction::rpc_types::IotaTransactionBlockEffects;
use iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI;
use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::crypto::SignatureScheme;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::types::TypeTag;
use iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use iota_interaction::IotaKeySignature;
use iota_interaction::OptionalSync;
use iota_interaction::IOTA_LOCAL_NETWORK_URL;
use lazy_static::lazy_static;
use secret_storage::Signer;
use serde::Deserialize;
use serde_json::Value;
use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::OnceCell;

use iota_sdk::IotaClient;
use iota_sdk::IotaClientBuilder;

static PACKAGE_ID: OnceCell<ObjectID> = OnceCell::const_new();
static CLIENT: OnceCell<TestClient> = OnceCell::const_new();

pub const TEST_GAS_BUDGET: u64 = 50_000_000;

lazy_static! {
    pub static ref TEST_COIN_TYPE: StructTag = "0x2::coin::Coin<bool>".parse().unwrap();
}

pub async fn get_funded_test_client<S: Signer<IotaKeySignature>, C: CoreClient<S>>(
    client: C,
    signer: S,
) -> anyhow::Result<TestClient<S, C>> {
    TestClient::new(client, signer).await
}

async fn init(iota_client: &IotaClient) -> anyhow::Result<ObjectID> {
    let network_id = iota_client.read_api().get_chain_identifier().await?;
    let address = get_active_address().await?;

    if let Ok(id) = std::env::var("IOTA_IDENTITY_PKG_ID").or(get_cached_id(&network_id).await) {
        std::env::set_var("IOTA_IDENTITY_PKG_ID", id.clone());
        id.parse().context("failed to parse object id from str")
    } else {
        publish_package(address).await
    }
}

async fn get_cached_id(network_id: &str) -> anyhow::Result<String> {
    let cache = tokio::fs::read_to_string(CACHED_PKG_ID).await?;
    let (cached_id, cached_network_id) = cache
        .split_once(';')
        .ok_or(anyhow!("Invalid or empty cached data"))?;

    if cached_network_id == network_id {
        Ok(cached_id.to_owned())
    } else {
        Err(anyhow!("A network change has invalidated the cached data"))
    }
}

async fn get_active_address() -> anyhow::Result<IotaAddress> {
    Command::new("iota")
        .arg("client")
        .arg("active-address")
        .arg("--json")
        .output()
        .await
        .context("Failed to execute command")
        .and_then(|output| Ok(serde_json::from_slice::<IotaAddress>(&output.stdout)?))
}

async fn publish_package(active_address: IotaAddress) -> anyhow::Result<ObjectID> {
    let output = Command::new("sh")
        .current_dir(SCRIPT_DIR)
        .arg("publish_identity_package.sh")
        .output()
        .await?;
    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        anyhow::bail!("Failed to publish move package: \n\n{stdout}\n\n{stderr}");
    }

    let package_id: ObjectID = {
        let stdout_trimmed = stdout.trim();
        ObjectID::from_str(stdout_trimmed).with_context(|| {
      let stderr = std::str::from_utf8(&output.stderr).unwrap();
      format!("failed to find IDENTITY_IOTA_PKG_ID in response from: '{stdout_trimmed}'; {stderr}")
    })?
    };

    // Persist package ID in order to avoid publishing the package for every test.
    let package_id_str = package_id.to_string();
    std::env::set_var("IDENTITY_IOTA_PKG_ID", package_id_str.as_str());
    let mut file = std::fs::File::create(CACHED_PKG_ID)?;
    write!(&mut file, "{};{}", package_id_str, active_address)?;

    Ok(package_id)
}

fn get_public_key_bytes(sender_public_jwk: &Jwk) -> Result<Vec<u8>, anyhow::Error> {
    let public_key_base_64 = &sender_public_jwk
        .try_okp_params()
        .map_err(|err| anyhow!("key not of type `Okp`; {err}"))?
        .x;

    identity_jose::jwu::decode_b64(public_key_base_64)
        .map_err(|err| anyhow!("could not decode base64 public key; {err}"))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GasObjectHelper {
    nanos_balance: u64,
}

async fn get_balance(address: IotaAddress) -> anyhow::Result<u64> {
    let output = Command::new("iota")
        .arg("client")
        .arg("gas")
        .arg("--json")
        .arg(address.to_string())
        .output()
        .await?;

    if !output.status.success() {
        let error_msg = String::from_utf8(output.stderr)?;
        anyhow::bail!("failed to get balance: {error_msg}");
    }

    let balance = serde_json::from_slice::<Vec<GasObjectHelper>>(&output.stdout)?
        .into_iter()
        .map(|gas_info| gas_info.nanos_balance)
        .sum();

    Ok(balance)
}

#[derive(Clone)]
pub struct TestClient<S: Signer<IotaKeySignature>, C: CoreClient<S>> {
    client: Arc<C>,
    signer: Arc<S>,
}

impl<S: Signer<IotaKeySignature>, C: CoreClient<S>> TestClient<S, C> {
    pub async fn new(client: C, signer: S) -> anyhow::Result<Self> {
        let active_address = get_active_address().await?;
        Self::new_from_address(client, signer, active_address).await
    }

    pub async fn new_from_address(
        product_client: C,
        signer: S,
        address: IotaAddress,
    ) -> anyhow::Result<Self> {
        let api_endpoint =
            std::env::var("API_ENDPOINT").unwrap_or_else(|_| IOTA_LOCAL_NETWORK_URL.to_string());
        let client = IotaClientBuilder::default().build(&api_endpoint).await?;
        let package_id = PACKAGE_ID
            .get_or_try_init(|| init(&client))
            .await
            .copied()?;

        let balance = get_balance(address).await?;
        if balance < TEST_GAS_BUDGET {
            request_funds(&address).await?;
        }

        Ok(TestClient {
            client: Arc::new(product_client),
            signer: Arc::new(signer),
        })
    }

    pub async fn new_with_key_type(
        client: C,
        signer: S,
        key_type: SignatureScheme,
    ) -> anyhow::Result<Self> {
        let address = make_address(key_type).await?;
        Self::new_from_address(client, signer, address).await
    }

    // Sets the current address to the address controller by this client.
    async fn switch_address(&self) -> anyhow::Result<()> {
        let output = Command::new("iota")
            .arg("client")
            .arg("switch")
            .arg("--address")
            .arg(self.client.sender_address().to_string())
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to switch address: {}",
                std::str::from_utf8(&output.stderr).unwrap()
            );
        }

        Ok(())
    }

    pub fn package_id(&self) -> ObjectID {
        self.client.package_id()
    }

    pub fn signer(&self) -> &KeytoolSigner {
        self.client.signer()
    }
}

#[cfg(feature = "transaction")]
pub async fn get_test_coin<S, C>(recipient: IotaAddress, client: &C) -> anyhow::Result<ObjectID>
where
    S: Signer<IotaKeySignature> + OptionalSync,
    C: CoreClient<S> + OptionalSync,
{
    TransactionBuilder::new(GetTestCoin { recipient })
        .build_and_execute(client)
        .await
        .context("failed to get test coins")
        .map(|tx_output| tx_output.output)
}

pub async fn make_address(key_type: SignatureScheme) -> anyhow::Result<IotaAddress> {
    if !matches!(
        key_type,
        SignatureScheme::ED25519 | SignatureScheme::Secp256k1 | SignatureScheme::Secp256r1
    ) {
        anyhow::bail!("key type {key_type} is not supported");
    }

    let output = Command::new("iota")
        .arg("client")
        .arg("new-address")
        .arg("--key-scheme")
        .arg(key_type.to_string())
        .arg("--json")
        .output()
        .await?;
    let new_address = {
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        let start_of_json = stdout.find('{').ok_or_else(|| {
            let stderr = std::str::from_utf8(&output.stderr).unwrap();
            anyhow!("No json in output: '{stdout}'; {stderr}",)
        })?;
        let json_result = serde_json::from_str::<Value>(stdout[start_of_json..].trim())?;
        let address_str = json_result
            .get("address")
            .context("no address in JSON output")?
            .as_str()
            .context("address is not a JSON string")?;

        address_str.parse()?
    };

    request_funds(&new_address).await?;

    Ok(new_address)
}

struct GetTestCoin {
    recipient: IotaAddress,
}

#[cfg(feature = "transaction")]
#[async_trait]
impl Transaction for GetTestCoin {
    type Output = ObjectID;
    type Error = Error;

    async fn build_programmable_transaction<C>(
        &self,
        _: &C,
    ) -> Result<ProgrammableTransaction, Self::Error>
    where
        C: CoreClientReadOnly + OptionalSync,
    {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let coin = ptb.programmable_move_call(
            IOTA_FRAMEWORK_PACKAGE_ID,
            ident_str!("coin").into(),
            ident_str!("zero").into(),
            vec![TypeTag::Bool],
            vec![],
        );
        ptb.transfer_args(self.recipient, vec![coin]);
        Ok(ptb.finish())
    }

    async fn apply<C>(
        self,
        effects: IotaTransactionBlockEffects,
        _client: &C,
    ) -> Result<Self::Output, Error>
    where
        C: CoreClientReadOnly + OptionalSync,
    {
        effects
            .created()
            .first()
            .map(|obj_ref| obj_ref.object_id())
            .ok_or_else(|| Error::TransactionUnexpectedResponse("no coins were created".to_owned()))
    }
}
