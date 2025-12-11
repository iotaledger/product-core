// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use fastcrypto::hash::HashFunction;
use iota_interaction::types::transaction::TransactionData;
use iota_interaction::IotaKeySignature;
use iota_keys::keystore::{AccountKeystore, InMemKeystore};
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::crypto::SignatureScheme;
use iota_sdk_types::crypto::Intent;
use secret_storage::{SignatureScheme as SignerSignatureScheme, Signer as SignerTrait};
use tokio::sync::RwLock;

pub struct InMemSigner {
  pub keystore: InMemKeystore,
  selected_alias: Arc<RwLock<String>>,
}

impl Default for InMemSigner {
  fn default() -> Self {
    Self::new()
  }
}

impl InMemSigner {
  /// Creates a new in memory signer with a random alias
  pub fn new() -> Self {
    let mut keystore = InMemKeystore::new_insecure_for_tests(0);
    let (address, _, _) = keystore
      .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
      .expect("Could not generate key");

    let alias = keystore.get_alias_by_address(&address).expect("Could not get alias");

    InMemSigner {
      keystore,
      selected_alias: Arc::new(RwLock::new(alias)),
    }
  }

  /// Creates a new in memory signer with a specific alias
  pub fn new_with_alias(alias: &str) -> Self {
    let mut keystore = InMemKeystore::new_insecure_for_tests(0);
    keystore
      .generate_and_add_new_key(SignatureScheme::ED25519, Some(alias.into()), None, None)
      .expect("Could not generate key");

    InMemSigner {
      keystore,
      selected_alias: Arc::new(RwLock::new(alias.into())),
    }
  }

  /// Selects an alias to be used for signing
  pub async fn select_alias(&self, alias: &str) -> anyhow::Result<()> {
    if !self.keystore.alias_exists(alias) {
      return Err(anyhow::anyhow!("Alias does not exist"));
    }

    let mut write = self.selected_alias.write().await;

    *write = alias.into();
    Ok(())
  }

  /// Returns the address of the selected alias
  pub async fn get_address(&self) -> anyhow::Result<IotaAddress> {
    let alias = self.selected_alias.read().await;
    let address = self.keystore.get_address_by_alias(alias.clone())?;
    Ok(*address)
  }

  /// Add a new alias to the keystore
  pub async fn add_alias(&mut self, alias: &str) -> anyhow::Result<(String, IotaAddress)> {
    let (address, _, _) = self
      .keystore
      .generate_and_add_new_key(SignatureScheme::ED25519, Some(alias.into()), None, None)
      .expect("Could not generate key");

    let alias = self
      .keystore
      .get_alias_by_address(&address)
      .expect("Could not get alias");

    Ok((alias, address))
  }
}

#[async_trait::async_trait]
impl SignerTrait<IotaKeySignature> for InMemSigner {
  type KeyId = ();

  async fn sign(
    &self,
    data: &TransactionData,
  ) -> secret_storage::Result<<IotaKeySignature as SignerSignatureScheme>::Signature> {
    use fastcrypto::hash::Blake2b256;
    let address = self.get_address().await.map_err(secret_storage::Error::Other)?;

    let tx_data_bcs = bcs::to_bytes(data)
      .context("Failed to serialize transaction data")
      .map_err(secret_storage::Error::Other)?;
    let intent_bytes = Intent::iota_transaction().to_bytes();
    let mut hasher = Blake2b256::default();
    hasher.update(intent_bytes);
    hasher.update(&tx_data_bcs);
    let digest = hasher.finalize().digest;

    let signature = self.keystore.sign_hashed(&address, &digest).unwrap();

    Ok(signature)
  }

  async fn public_key(
    &self,
  ) -> secret_storage::Result<<IotaKeySignature as secret_storage::SignatureScheme>::PublicKey> {
    let address = self.get_address().await.map_err(secret_storage::Error::Other)?;
    let res = self.keystore.get_key(&address).unwrap();

    Ok(res.public())
  }

  fn key_id(&self) -> Self::KeyId {}
}
