// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use crate::product_client::AbstractProductClient;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "@iota/iota-sdk/client")]
extern "C" {
    #[wasm_bindgen(js_name = "getFullnodeUrl")]
    fn get_fullnode_url(network_name: &str) -> String;

    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = IotaClient)]
    pub type TsIotaClient;

    #[wasm_bindgen(constructor)]
    pub fn _new(arguments: TsIotaClientNewArguments) -> TsIotaClient;
}

#[derive(Clone)]
#[wasm_bindgen(skip_typescript, getter_with_clone)]
pub struct TsIotaClientNewArguments {
    pub url: String,
}

impl TsIotaClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self::_new(TsIotaClientNewArguments { url: url.into() })
    }
}

impl From<AbstractProductClient> for TsIotaClient {
    fn from(client: AbstractProductClient) -> Self {
        let network = client.network;
        let network_name = if network.is_custom() {
            Cow::Borrowed("localnet")
        } else {
            Cow::Owned(network.to_string())
        };

        let rpc_url = get_fullnode_url(&network_name);
        Self::new(rpc_url)
    }
}
