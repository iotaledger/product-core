// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use iota_sdk::graphql_client::Client;
use iota_sdk::types::ObjectId;
use product_core::{network::Network, product_client::ProductClient};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[wasm_bindgen(skip_typescript)]
pub struct WasmIotaClient(pub(crate) Client);

#[wasm_bindgen(module = "@iota/iota-interaction-ts/core-client")]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = ProductClient)]
    pub type WasmProductClient;

    #[wasm_bindgen(method, getter)]
    pub fn network(this: &WasmProductClient) -> String;
    #[wasm_bindgen(method, getter, js_name = packageId)]
    pub fn package_id(this: &WasmProductClient) -> String;
    #[wasm_bindgen(method)]
    pub(crate) fn iota_client(this: &WasmProductClient) -> WasmIotaClient;
}

/// A type implementing [ProductClient] which can be constructed from an arbitrary [WasmProductClient].
#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub(crate) struct AbstractProductClient {
    pub(crate) network: Network,
    pub(crate) package_id: ObjectId,
    pub(crate) iota_client: Client,
}

#[wasm_bindgen(js_class = AbstractProductClient)]
impl AbstractProductClient {
    #[wasm_bindgen(getter = network)]
    pub fn network(&self) -> String {
        self.network.to_string()
    }

    #[wasm_bindgen(getter = packageId)]
    pub fn package_id(&self) -> String {
        self.package_id.to_string()
    }

    pub fn iota_client(&self) -> WasmIotaClient {
        WasmIotaClient(self.iota_client.clone())
    }
}

impl AbstractProductClient {
    pub(crate) fn new(client: &impl ProductClient) -> Self {
        Self {
            network: client.network(),
            package_id: client.package_id(),
            iota_client: (*client).clone(),
        }
    }
}

impl Deref for AbstractProductClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.iota_client
    }
}

impl ProductClient for AbstractProductClient {
    fn network(&self) -> Network {
        self.network
    }

    fn package_id(&self) -> ObjectId {
        self.package_id
    }
}

impl TryFrom<WasmProductClient> for AbstractProductClient {
    type Error = JsError;
    fn try_from(wasm_client: WasmProductClient) -> Result<Self, Self::Error> {
        let network = wasm_client.network().parse()?;
        let package_id = wasm_client.package_id().parse()?;
        let iota_client = wasm_client.iota_client().0;

        Ok(Self {
            network,
            package_id,
            iota_client,
        })
    }
}
