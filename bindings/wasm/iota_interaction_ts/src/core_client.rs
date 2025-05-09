
use wasm_bindgen::prelude::wasm_bindgen;
use crate::bindings::{WasmIotaClient, WasmTransactionSigner};
use crate::WasmPublicKey;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = CoreClientReadOnly)]
    pub type WasmCoreClientReadOnly;

    #[wasm_bindgen(method, js_name = packageId)]
    fn package_id(this: &WasmCoreClientReadOnly) -> String;

    #[wasm_bindgen(method, js_name = network)]
    fn network(this: &WasmCoreClientReadOnly) -> String;

    #[wasm_bindgen(method, js_name = iotaClient)]
    fn iota_client(this: &WasmCoreClientReadOnly) -> WasmIotaClient;

    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = CoreClient, extends = WasmCoreClientReadOnly)]
    pub type WasmCoreClient;

    #[wasm_bindgen(method)]
    fn signer(this: &WasmCoreClient) -> WasmTransactionSigner;

    #[wasm_bindgen(method, js_name = senderAddress)]
    fn sender_address(this: &WasmCoreClient) -> String;

    #[wasm_bindgen(method, js_name = senderPublicKey)]
    fn sender_public_key(this: &WasmCoreClient) -> WasmPublicKey;
}
