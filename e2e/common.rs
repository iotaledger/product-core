// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;

use anyhow::anyhow;
use anyhow::Context;
use identity_iota_core::IotaDID;
use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;
use identity_storage::JwkMemStore;
use identity_storage::JwkStorage;
use identity_storage::KeyId;
use identity_storage::KeyIdMemstore;
use identity_storage::KeyIdStorage;
use identity_storage::KeyType;
use identity_storage::MethodDigest;
use identity_storage::Storage;
use identity_storage::StorageSigner;
use identity_iota_core::rebased::client::IdentityClient;
use identity_iota_core::rebased::client::IdentityClientReadOnly;
use identity_iota_core::rebased::client::IotaKeySignature;
use identity_iota_core::rebased::transaction::Transaction;
use identity_iota_core::rebased::utils::request_funds;
use identity_verification::VerificationMethod;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;
use iota_sdk::IotaClient;
use iota_sdk::IotaClientBuilder;
use jsonpath_rust::JsonPathQuery;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use secret_storage::Signer;
use serde_json::Value;
use tokio::process::Command;
use tokio::sync::OnceCell;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;
pub type MemSigner<'s> = StorageSigner<'s, JwkMemStore, KeyIdMemstore>;

static PACKAGE_ID: OnceCell<ObjectID> = OnceCell::const_new();
static CLIENT: OnceCell<TestClient> = OnceCell::const_new();
const SCRIPT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/");
const CACHED_PKG_ID: &str = "/tmp/identity_iota_pkg_id.txt";

pub const TEST_GAS_BUDGET: u64 = 50_000_000;
pub const TEST_DOC: &[u8] = &[
  68, 73, 68, 1, 0, 131, 1, 123, 34, 100, 111, 99, 34, 58, 123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58,
  48, 34, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91,
  123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58, 48, 35, 79, 115, 55, 95, 66, 100, 74, 120, 113, 86, 119,
  101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100, 108, 74, 73, 99, 48,
  45, 100, 50, 49, 73, 34, 44, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58, 48,
  58, 48, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 74, 115, 111, 110, 87, 101, 98, 75, 101, 121, 34, 44, 34, 112,
  117, 98, 108, 105, 99, 75, 101, 121, 74, 119, 107, 34, 58, 123, 34, 107, 116, 121, 34, 58, 34, 79, 75, 80, 34, 44,
  34, 97, 108, 103, 34, 58, 34, 69, 100, 68, 83, 65, 34, 44, 34, 107, 105, 100, 34, 58, 34, 79, 115, 55, 95, 66, 100,
  74, 120, 113, 86, 119, 101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100,
  108, 74, 73, 99, 48, 45, 100, 50, 49, 73, 34, 44, 34, 99, 114, 118, 34, 58, 34, 69, 100, 50, 53, 53, 49, 57, 34, 44,
  34, 120, 34, 58, 34, 75, 119, 99, 54, 89, 105, 121, 121, 65, 71, 79, 103, 95, 80, 116, 118, 50, 95, 49, 67, 80, 71,
  52, 98, 86, 87, 54, 102, 89, 76, 80, 83, 108, 115, 57, 112, 122, 122, 99, 78, 67, 67, 77, 34, 125, 125, 93, 125, 44,
  34, 109, 101, 116, 97, 34, 58, 123, 34, 99, 114, 101, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50, 52, 45, 48, 53, 45,
  50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 44, 34, 117, 112, 100, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50,
  52, 45, 48, 53, 45, 50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 125, 125,
];
pub static TEST_COIN_TYPE: LazyLock<StructTag> = LazyLock::new(|| "0x2::coin::Coin<bool>".parse().unwrap());

pub async fn get_client() -> anyhow::Result<TestClient> {
  let client = IotaClientBuilder::default().build_localnet().await?;
  let package_id = PACKAGE_ID.get_or_try_init(|| init(&client)).await.copied()?;
  let address = get_active_address().await?;

  request_funds(&address).await?;

  let storage = Arc::new(Storage::new(JwkMemStore::new(), KeyIdMemstore::new()));
  let identity_client = IdentityClientReadOnly::new(client, package_id).await?;

  Ok(TestClient {
    client: identity_client,
    package_id,
    address,
    storage,
  })
}

async fn init(iota_client: &IotaClient) -> anyhow::Result<ObjectID> {
  let network_id = iota_client.read_api().get_chain_identifier().await?;
  let address = get_active_address().await?;

  if let Ok(id) = std::env::var("IDENTITY_IOTA_PKG_ID").or(get_cached_id(&network_id).await) {
    std::env::set_var("IDENTITY_IOTA_PKG_ID", id.clone());
    id.parse().context("failed to parse object id from str")
  } else {
    publish_package(address).await
  }
}

async fn get_cached_id(network_id: &str) -> anyhow::Result<String> {
  let cache = tokio::fs::read_to_string(CACHED_PKG_ID).await?;
  let (cached_id, cached_network_id) = cache.split_once(';').ok_or(anyhow!("Invalid or empty cached data"))?;

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

  if !output.status.success() {
    anyhow::bail!(
      "Failed to publish move package: \n\n{}\n\n{}",
      std::str::from_utf8(&output.stdout).unwrap(),
      std::str::from_utf8(&output.stderr).unwrap()
    );
  }

  let publish_result = {
    let output_str = std::str::from_utf8(&output.stdout).unwrap();
    let start_of_json = output_str.find('{').ok_or(anyhow!("No json in output"))?;
    serde_json::from_str::<Value>(output_str[start_of_json..].trim())?
  };

  let package_id = publish_result
    .path("$.objectChanges[?(@.type == 'published')].packageId")
    .map_err(|e| anyhow!("Failed to parse JSONPath: {e}"))
    .and_then(|value| Ok(serde_json::from_value::<Vec<ObjectID>>(value)?))?
    .first()
    .copied()
    .ok_or_else(|| anyhow!("Failed to parse package ID after publishing"))?;

  // Persist package ID in order to avoid publishing the package for every test.
  let package_id_str = package_id.to_string();
  std::env::set_var("IDENTITY_IOTA_PKG_ID", package_id_str.as_str());
  let mut file = std::fs::File::create(CACHED_PKG_ID)?;
  write!(&mut file, "{};{}", package_id_str, active_address)?;

  Ok(package_id)
}

pub async fn get_key_data() -> Result<(Storage<JwkMemStore, KeyIdMemstore>, KeyId, Jwk, Vec<u8>), anyhow::Error> {
  let storage = Storage::<JwkMemStore, KeyIdMemstore>::new(JwkMemStore::new(), KeyIdMemstore::new());
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key_jwk = generate.jwk.to_public().expect("public components should be derivable");
  let public_key_bytes = get_public_key_bytes(&public_key_jwk)?;
  // let sender_address = convert_to_address(&public_key_bytes)?;

  Ok((storage, generate.key_id, public_key_jwk, public_key_bytes))
}

fn get_public_key_bytes(sender_public_jwk: &Jwk) -> Result<Vec<u8>, anyhow::Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| anyhow!("key not of type `Okp`; {err}"))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64).map_err(|err| anyhow!("could not decode base64 public key; {err}"))
}

#[derive(Clone)]
pub struct TestClient {
  client: IdentityClientReadOnly,
  package_id: ObjectID,
  #[allow(dead_code)]
  address: IotaAddress,
  storage: Arc<MemStorage>,
}

impl Deref for TestClient {
  type Target = IdentityClientReadOnly;
  fn deref(&self) -> &Self::Target {
    &self.client
  }
}

impl TestClient {
  // Sets the current address to the address controller by this client.
  async fn switch_address(&self) -> anyhow::Result<()> {
    let output = Command::new("iota")
      .arg("client")
      .arg("switch")
      .arg("--address")
      .arg(self.address.to_string())
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
    self.package_id
  }

  pub async fn new_address(&self) -> anyhow::Result<Self> {
    let output = Command::new("iota")
      .arg("client")
      .arg("new-address")
      .arg("ed25519")
      .arg("--json")
      .output()
      .await?;
    let new_address = {
      let output_str = std::str::from_utf8(&output.stdout).unwrap();
      let start_of_json = output_str.find('{').ok_or(anyhow!("No json in output"))?;
      let json_result = serde_json::from_str::<Value>(output_str[start_of_json..].trim())?;
      let address_json = json_result
        .path("$.address")
        .map_err(|e| anyhow!("failed to parse json output: {e}"))?;
      serde_json::from_value::<IotaAddress>(address_json)?
    };

    request_funds(&new_address).await?;

    let mut new_client = self.clone();
    new_client.address = new_address;
    Ok(new_client)
  }

  pub async fn new_user_client(&self) -> anyhow::Result<IdentityClient<MemSigner>> {
    let generate = self
      .storage
      .key_storage()
      .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
      .await?;
    let public_key_jwk = generate.jwk.to_public().expect("public components should be derivable");
    let signer = StorageSigner::new(&self.storage, generate.key_id, public_key_jwk);

    let user_client = IdentityClient::new(self.client.clone(), signer).await?;

    request_funds(&user_client.sender_address()).await?;

    Ok(user_client)
  }

  pub async fn store_key_id_for_verification_method(
    &self,
    identity_client: IdentityClient<StorageSigner<'_, JwkMemStore, KeyIdMemstore>>,
    did: IotaDID,
  ) -> anyhow::Result<()> {
    let public_key = identity_client.signer().public_key();
    let key_id = identity_client.signer().key_id();
    let fragment = key_id.as_str();
    let method = VerificationMethod::new_from_jwk(did, public_key.clone(), Some(fragment))?;
    let method_digest: MethodDigest = MethodDigest::new(&method)?;

    self
      .storage
      .key_id_storage()
      .insert_key_id(method_digest, key_id.clone())
      .await?;

    Ok(())
  }
}

pub async fn get_test_coin<S>(recipient: IotaAddress, client: &IdentityClient<S>) -> anyhow::Result<ObjectID>
where
  S: Signer<IotaKeySignature> + Sync,
{
  let mut ptb = ProgrammableTransactionBuilder::new();
  let coin = ptb.programmable_move_call(
    IOTA_FRAMEWORK_PACKAGE_ID,
    ident_str!("coin").into(),
    ident_str!("zero").into(),
    vec![TypeTag::Bool],
    vec![],
  );
  ptb.transfer_args(recipient, vec![coin]);
  ptb
    .finish()
    .execute(client)
    .await?
    .response
    .effects
    .expect("tx should have had effects")
    .created()
    .first()
    .map(|obj| obj.object_id())
    .context("no coins were created")
}
