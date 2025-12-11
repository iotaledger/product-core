// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;
use secret_storage::{Error as SecretStorageError, Signer};

use crate::types::base_types::IotaAddress;
use crate::types::crypto::{IotaKeyPair, PublicKey, Signature};
use crate::types::transaction::TransactionData;
use crate::IotaKeySignature;

/// A wrapper over an [IotaKeyPair] that implements [Signer<IotaKeySignature>].
#[derive(Debug)]
pub struct KeyPairSigner(IotaKeyPair);

impl Clone for KeyPairSigner {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl AsRef<IotaKeyPair> for KeyPairSigner {
  fn as_ref(&self) -> &IotaKeyPair {
    &self.0
  }
}

impl KeyPairSigner {
  /// Returns a new [KeyPairSigner] from the given [IotaKeyPair].
  pub fn new(keypair: IotaKeyPair) -> Self {
    Self(keypair)
  }

  /// Consumes this [KeyPairSigner] returning the wrapped [IotaKeyPair].
  pub fn into_inner(self) -> IotaKeyPair {
    self.0
  }

  /// Returns the [PublicKey] of this wrapped [IotaKeyPair].
  pub fn public_key(&self) -> PublicKey {
    self.0.public()
  }
}

#[cfg_attr(feature = "send-sync-transaction", async_trait)]
#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
impl Signer<IotaKeySignature> for KeyPairSigner {
  type KeyId = IotaAddress;

  fn key_id(&self) -> Self::KeyId {
    IotaAddress::from(&self.0.public())
  }

  async fn public_key(&self) -> Result<PublicKey, SecretStorageError> {
    Ok(self.public_key())
  }

  async fn sign(&self, data: &TransactionData) -> Result<Signature, SecretStorageError> {
    use fastcrypto::hash::{Blake2b256, HashFunction};
    use fastcrypto::traits::Signer;
    use iota_sdk_types::crypto::intent::Intent;

    let tx_data_bcs =
      bcs::to_bytes(data).map_err(|e| SecretStorageError::Other(anyhow!("bcs serialization failed: {e}")))?;
    let intent_bytes = Intent::iota_transaction().to_bytes();
    let mut hasher = Blake2b256::default();
    hasher.update(intent_bytes);
    hasher.update(&tx_data_bcs);
    let digest = hasher.finalize().digest;

    Ok(self.0.sign(&digest))
  }
}
