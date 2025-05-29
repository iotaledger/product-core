// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::traits::EncodeDecodeBase64;
use iota_interaction::KeyPairSigner;
use secret_storage::Signer as _;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

use crate::bindings::WasmKeyPair;
use crate::error::Result;
use crate::WasmPublicKey;

/// A wrapper over an {@link @iota/iota-sdk/cryptography/KeyPair | IOTA KeyPair}
/// that implements {@link TransactionSigner}.
#[derive(Clone)]
#[wasm_bindgen(js_name = KeyPairSigner)]
pub struct WasmKeyPairSigner(pub(crate) KeyPairSigner);

#[wasm_bindgen(js_class = KeyPairSigner)]
impl WasmKeyPairSigner {
  /// Returns a new {@link KeyPairSigner} from the given
  /// {@link @iota/iota-sdk/cryptography/KeyPair | IOTA KeyPair}.
  #[wasm_bindgen(constructor)]
  pub fn new(wasm_keypair: &WasmKeyPair) -> Result<Self> {
    let keypair = wasm_keypair.try_into()?;

    Ok(WasmKeyPairSigner(KeyPairSigner::new(keypair)))
  }

  // Implementation of Signer<IotaKeySignature> interface.

  #[wasm_bindgen]
  pub async fn sign(&self, tx_data_bcs: &[u8]) -> Result<String> {
    let tx_data = bcs::from_bytes(tx_data_bcs).map_err(|e| JsError::new(&e.to_string()))?;
    self
      .0
      .sign(&tx_data)
      .await
      .map(|sig| sig.encode_base64())
      .map_err(|e| JsError::new(&e.to_string()).into())
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub async fn public_key(&self) -> Result<WasmPublicKey> {
    WasmPublicKey::try_from(&self.0.public_key())
  }

  #[wasm_bindgen(js_name = keyId)]
  pub fn key_id(&self) -> String {
    self.0.key_id().to_string()
  }

  #[wasm_bindgen(js_name = iotaPublicKeyBytes)]
  pub fn iota_public_key_bytes(&self) -> Vec<u8> {
    let pk = self.0.public_key();
    let flag = pk.flag();

    [&[flag], pk.as_ref()].concat()
  }
}
