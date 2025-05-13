// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use iota_interaction::move_types::language_storage::StructTag;
use iota_interaction::rpc_types::{IotaTransactionBlockEffects, IotaTransactionBlockEffectsAPI};
use iota_interaction::types::base_types::{IotaAddress, ObjectID};
use iota_interaction::types::crypto::SignatureScheme;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::types::{TypeTag, IOTA_FRAMEWORK_PACKAGE_ID};
use iota_interaction::{ident_str, IotaClientTrait, IotaKeySignature, IotaTransactionBlockEffectsMutAPI, OptionalSync};
use iota_sdk::rpc_types::{IotaObjectDataOptions, IotaObjectResponse};
use iota_sdk::types::object::Owner;
use iota_sdk::IotaClient;
use lazy_static::lazy_static;
use secret_storage::Signer;
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;
use tokio::sync::OnceCell;

use super::utils::request_funds;
use crate::core_client::{CoreClient, CoreClientReadOnly};
#[cfg(feature = "transaction")]
use crate::transaction::transaction_builder::Transaction;
#[cfg(feature = "transaction")]
use crate::transaction::transaction_builder::TransactionBuilder;
use crate::Error;

/// Directory containing the scripts used for testing.
///
/// Default value is the `scripts` directory relative to the current crate.
pub const SCRIPT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/publish_package.sh");

/// Cached package file for the product related package.
pub const CACHED_PKG_FILE: &str = "/tmp/product_pkg_id.txt";

pub const TEST_GAS_BUDGET: u64 = 50_000_000;

lazy_static! {
  pub static ref TEST_COIN_TYPE: StructTag = "0x2::coin::Coin<bool>".parse().unwrap();
}

/// Initializes the package ID for the product related package.
///
/// # Arguments
///
/// * `iota_client` - The IotaClient to use for the request.
/// * `cached_pkg_file` - The path to the cached package file (optional).
/// * `script_file` - The path to the script file (optional).
///
/// If the optional arguments are not provided, the default values will be used.
/// The default values are:
/// * `cached_pkg_file`: [`CACHED_PKG_FILE`]
/// * `script_file`: [`SCRIPT_FILE`]
///
/// # Returns
///
/// * `Ok(ObjectID)` - The package ID.
///
/// # Errors
///
/// * `anyhow::Error` - An error occurred.
pub async fn init_product_package(
  iota_client: &IotaClient,
  cached_pkg_file: Option<&str>,
  script_file: Option<&str>,
) -> anyhow::Result<ObjectID> {
  let network_id = iota_client.read_api().get_chain_identifier().await?;
  let address = get_active_address().await?;

  if let Ok(id) = get_cached_id(&network_id, cached_pkg_file).await {
    std::env::set_var("PRODUCT_IOTA_PKG_ID", id.clone());
    id.parse().context("failed to parse object id from str")
  } else {
    publish_package(
      address,
      script_file.unwrap_or(SCRIPT_FILE),
      cached_pkg_file.unwrap_or(CACHED_PKG_FILE),
    )
    .await
  }
}

/// Retrieves the cached package ID for the given network.
///
/// # Arguments
///
/// * `network_id` - The network identifier.
/// * `cached_pkg_file` - The path to the cached package file.
///
/// # Returns
///
/// * `Ok(String)` - The cached package ID.
///
/// # Errors
///
/// * `anyhow::Error` - An error occurred.
pub async fn get_cached_id(network_id: &str, cached_pkg_file: Option<&str>) -> anyhow::Result<String> {
  let cache = tokio::fs::read_to_string(cached_pkg_file.unwrap_or(CACHED_PKG_FILE)).await?;
  let (cached_id, cached_network_id) = cache.split_once(';').ok_or(anyhow!("Invalid or empty cached data"))?;

  if cached_network_id == network_id {
    Ok(cached_id.to_owned())
  } else {
    Err(anyhow!("A network change has invalidated the cached data"))
  }
}

/// Retrieves the active address.
///
/// # Returns
///
/// * `Ok(IotaAddress)` - The active address.
///
/// # Errors
///
/// * `anyhow::Error` - An error occurred.
pub async fn get_active_address() -> anyhow::Result<IotaAddress> {
  Command::new("iota")
    .arg("client")
    .arg("active-address")
    .arg("--json")
    .output()
    .await
    .context("Failed to execute command")
    .and_then(|output| Ok(serde_json::from_slice::<IotaAddress>(&output.stdout)?))
}

/// Publishes the product package.
///
/// # Arguments
///
/// * `active_address` - The active address.
/// * `script_file` - The path to the script file.
/// * `cached_pkg_file` - The path to the cached package file.
///
/// # Returns
///
/// * `Ok(ObjectID)` - The package ID.
///
/// # Errors
///
/// * `anyhow::Error` - An error occurred.
pub async fn publish_package(
  active_address: IotaAddress,
  script_file: &str,
  cached_pkg_file: &str,
) -> anyhow::Result<ObjectID> {
  let output = Command::new("sh").arg(script_file).output().await?;
  let stdout = std::str::from_utf8(&output.stdout).unwrap();

  if !output.status.success() {
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    anyhow::bail!("Failed to publish move package: \n\n{stdout}\n\n{stderr}");
  }

  let package_id: ObjectID = {
    let stdout_trimmed = stdout.trim();
    ObjectID::from_str(stdout_trimmed).with_context(|| {
      let stderr = std::str::from_utf8(&output.stderr).unwrap();
      format!("failed to find PRODUCT_IOTA_PKG_ID in response from: '{stdout_trimmed}'; {stderr}")
    })?
  };

  // Persist package ID in order to avoid publishing the package for every test.
  let package_id_str = package_id.to_string();
  std::env::set_var("PRODUCT_IOTA_PKG_ID", package_id_str.as_str());
  std::fs::File::create(cached_pkg_file)
    .context("failed to create cached pkg file")?
    .write_all(format!("{};{}", package_id_str, active_address).as_bytes())
    .context("failed to write cached pkg file")?;

  Ok(package_id)
}

/// Helper struct for deserializing the output of the `iota client gas` command.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GasObjectHelper {
  nanos_balance: u64,
}

/// Retrieves the balance of the given address.
pub async fn get_balance(address: IotaAddress) -> anyhow::Result<u64> {
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

/// Retrieves a test coin for the given recipient.
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

/// Creates a new address and requests funds for it.
///
/// # Arguments
///
/// * `key_type` - The type of the key to use for the address.
///
/// # Returns
///
/// * `Ok(IotaAddress)` - The new address.
///
/// # Errors
///
/// * `anyhow::Error` - An error occurred.
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

/// A transaction that creates a coin for a given recipient.
#[derive(Debug, Clone)]
pub struct GetTestCoin {
  recipient: IotaAddress,
}

#[cfg(feature = "transaction")]
#[async_trait]
impl Transaction for GetTestCoin {
  type Output = ObjectID;

  type Error = Error;

  async fn build_programmable_transaction<C>(&self, _client: &C) -> Result<ProgrammableTransaction, Self::Error>
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
    mut effects: IotaTransactionBlockEffects,
    client: &C,
  ) -> (Result<Self::Output, Self::Error>, IotaTransactionBlockEffects)
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    let created_objects = effects
      .created()
      .iter()
      .enumerate()
      .filter(|(_, obj)| matches!(obj.owner, Owner::AddressOwner(address) if address == self.recipient))
      .map(|(i, obj_ref)| (i, obj_ref.object_id()));

    let is_target_coin =
      |obj_info: &IotaObjectResponse| obj_info.data.as_ref().unwrap().type_.as_ref().unwrap().is_coin();

    let mut i = None;
    let mut id = None;
    for (pos, obj) in created_objects {
      let coin_info = client
        .client_adapter()
        .read_api()
        .get_object_with_options(obj, IotaObjectDataOptions::new().with_type())
        .await;
      match coin_info {
        Ok(info) if is_target_coin(&info) => {
          i = Some(pos);
          id = Some(obj);
          break;
        }
        _ => continue,
      }
    }

    if let (Some(i), Some(id)) = (i, id) {
      effects.created_mut().swap_remove(i);
      (Ok(id), effects)
    } else {
      (
        Err(Error::TransactionUnexpectedResponse(format!(
          "transaction didn't create any coins for address {}",
          self.recipient
        ))),
        effects,
      )
    }
  }
}
