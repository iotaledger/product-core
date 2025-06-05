// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::wasm_bindgen;

use crate::bindings::{WasmIotaClient, WasmTransactionSigner};
use crate::WasmPublicKey;

#[wasm_bindgen(module = "@iota/iota-interaction-ts/core_client")]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = CoreClientReadOnly)]
  pub type WasmCoreClientReadOnly;

  #[wasm_bindgen(method, js_name = packageId)]
  pub fn package_id(this: &WasmCoreClientReadOnly) -> String;

  #[wasm_bindgen(method, js_name = packageHistory)]
  pub fn package_history(this: &WasmCoreClientReadOnly) -> Vec<String>;

  #[wasm_bindgen(method, js_name = network)]
  pub fn network(this: &WasmCoreClientReadOnly) -> String;

  #[wasm_bindgen(method, js_name = iotaClient)]
  pub fn iota_client(this: &WasmCoreClientReadOnly) -> WasmIotaClient;

  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = CoreClient, extends = WasmCoreClientReadOnly)]
  pub type WasmCoreClient;

  #[wasm_bindgen(method)]
  pub fn signer(this: &WasmCoreClient) -> WasmTransactionSigner;

  #[wasm_bindgen(method, js_name = senderAddress)]
  pub fn sender_address(this: &WasmCoreClient) -> String;

  #[wasm_bindgen(method, js_name = senderPublicKey)]
  pub fn sender_public_key(this: &WasmCoreClient) -> WasmPublicKey;
}
