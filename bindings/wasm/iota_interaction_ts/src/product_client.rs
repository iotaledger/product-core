// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{graphql_client::Client, types::ObjectId};
use product_core::{
    network::Network, product_client::ProductClient, type_origin_table::TypeOriginTable,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "@iota/iota-interaction-ts/core-client")]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = ProductClient)]
    pub type WasmProductClient;

    #[wasm_bindgen(method, getter)]
    pub fn network(this: &WasmProductClient) -> String;
    #[wasm_bindgen(method, getter, js_name = packageId)]
    pub fn package_id(this: &WasmProductClient) -> String;
    #[wasm_bindgen(method, getter, js_name = "_typeOriginTable")]
    pub fn type_origin_table(this: &WasmProductClient) -> WasmTypeOriginTable;
    #[wasm_bindgen(method, getter, js_name = "_iotaClient")]
    pub fn iota_client(this: &WasmProductClient) -> WasmIotaClient;
}

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct WasmIotaClient(pub(crate) Client);

impl AsRef<Client> for WasmIotaClient {
    fn as_ref(&self) -> &Client {
        &self.0
    }
}

impl From<Client> for WasmIotaClient {
    fn from(value: Client) -> Self {
        Self(value)
    }
}

impl From<WasmIotaClient> for Client {
    fn from(value: WasmIotaClient) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct WasmTypeOriginTable(pub(crate) TypeOriginTable);

impl AsRef<TypeOriginTable> for WasmTypeOriginTable {
    fn as_ref(&self) -> &TypeOriginTable {
        &self.0
    }
}

impl From<TypeOriginTable> for WasmTypeOriginTable {
    fn from(value: TypeOriginTable) -> Self {
        Self(value)
    }
}

impl From<WasmTypeOriginTable> for TypeOriginTable {
    fn from(value: WasmTypeOriginTable) -> Self {
        value.0
    }
}

/// A type implementing [ProductClient] which can be constructed from an
/// arbitrary [WasmProductClient].
#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct AbstractProductClient {
    pub(crate) network: Network,
    pub(crate) package_id: ObjectId,
    pub(crate) iota_client: Client,
    pub(crate) type_origin_table: TypeOriginTable,
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

    #[wasm_bindgen(getter, js_name = _iotaClient)]
    pub fn iota_client(&self) -> WasmIotaClient {
        WasmIotaClient(self.iota_client.clone())
    }

    #[wasm_bindgen(getter, js_name = _typeOriginTable)]
    pub fn type_origin_table(&self) -> WasmTypeOriginTable {
        WasmTypeOriginTable(self.type_origin_table.clone())
    }
}

impl AbstractProductClient {
    pub fn new(client: &impl ProductClient) -> Self {
        Self {
            network: client.network(),
            package_id: client.package_id(),
            iota_client: client.as_ref().clone(),
            type_origin_table: client.type_origin_table().clone(),
        }
    }
}

impl AsRef<Client> for AbstractProductClient {
    fn as_ref(&self) -> &Client {
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

    fn type_origin_table(&self) -> &TypeOriginTable {
        &self.type_origin_table
    }
}

impl From<WasmProductClient> for AbstractProductClient {
    fn from(wasm_client: WasmProductClient) -> Self {
        let network = wasm_client
            .network()
            .parse()
            .expect("invalid value for ProductClient.network");
        let package_id = wasm_client
            .package_id()
            .parse()
            .expect("invalid value for ProductClient.packageId");
        let iota_client = wasm_client.iota_client().0;
        let type_origin_table = wasm_client.type_origin_table().0;

        Self {
            network,
            package_id,
            iota_client,
            type_origin_table,
        }
    }
}
