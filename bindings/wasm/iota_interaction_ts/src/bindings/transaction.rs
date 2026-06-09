// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Error as JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::bindings::iota_client::TsIotaClient;

#[wasm_bindgen(module = "@iota/iota-sdk/transactions")]
extern "C" {
    #[wasm_bindgen(typescript_type = Transaction)]
    #[derive(Clone)]
    pub type WasmTransaction;

    #[wasm_bindgen(static_method_of = WasmTransaction, js_name = from, catch)]
    pub fn from_bcs_bytes(bytes: &[u8]) -> Result<WasmTransaction, JsError>;

    #[wasm_bindgen(method, catch, js_name = build)]
    async fn _build(this: &WasmTransaction, args: BuildArguments) -> Result<Vec<u8>, JsError>;
}

impl WasmTransaction {
    /// Builds a transaction into its BCS format. Any incomplete value is fetched through
    /// the provided client.
    pub async fn build(self, client: impl Into<TsIotaClient>) -> Result<Vec<u8>, JsError> {
        self._build(BuildArguments::new(client)).await
    }
}

#[derive(Default, Clone)]
#[wasm_bindgen(skip_typescript, getter_with_clone)]
pub struct BuildArguments {
    pub client: Option<TsIotaClient>,
    #[wasm_bindgen(js_name = "onlyTransactionKind")]
    pub only_transaction_kind: Option<bool>,
    #[wasm_bindgen(js_name = "maxSizeBytes")]
    pub max_size_bytes: Option<usize>,
}

impl BuildArguments {
    pub fn new(client: impl Into<TsIotaClient>) -> Self {
        Self {
            client: Some(client.into()),
            ..Default::default()
        }
    }
}
