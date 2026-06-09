// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    transaction_builder::TransactionSigner,
    types::{Address, Transaction, UserSignature},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@iota/iota-sdk/cryptography")]
extern "C" {
    #[wasm_bindgen(typescript_type = SignatureWithBytes)]
    pub type WasmSignatureWithBytes;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &WasmSignatureWithBytes) -> String;

    #[wasm_bindgen(typescript_type = "Signer")]
    pub type WasmTransactionSigner;

    #[wasm_bindgen(method, catch, js_name = signTransaction)]
    pub async fn sign_transaction(
        this: &WasmTransactionSigner,
        tx_bytes: &[u8],
    ) -> Result<WasmSignatureWithBytes, js_sys::Error>;

    #[wasm_bindgen(method, getter, js_name = toIotaAddress)]
    pub fn to_iota_address(this: &WasmTransactionSigner) -> String;
}

#[derive(Debug, thiserror::Error)]
#[error("{}", self.0.message())]
pub struct WasmTransactionSignerError(js_sys::Error);

impl From<js_sys::Error> for WasmTransactionSignerError {
    fn from(value: js_sys::Error) -> Self {
        Self(value)
    }
}

impl TransactionSigner for WasmTransactionSigner {
    type Error = WasmTransactionSignerError;
    async fn sign(&self, transaction: &Transaction) -> Result<UserSignature, Self::Error> {
        let base64_sig = self
            .sign_transaction(&transaction.to_bcs())
            .await?
            .signature();
        Ok(UserSignature::from_base64(&base64_sig).expect("TS SDK produces valid signatures"))
    }
}

impl secret_storage::iota::TransactionSigner for WasmTransactionSigner {
    fn address(&self) -> Address {
        Address::from_hex(self.to_iota_address()).expect("TS SDK Signer returns valid addresses")
    }
}
