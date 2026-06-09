// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, time::Duration};

use http::HeaderMap;
use product_core::operation::{GasStationOptions, OperationBuilder, OperationOutput};
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

use crate::{
    bindings::{transaction::WasmTransaction, transaction_signer::WasmTransactionSigner},
    operation::WasmOperation,
    product_client::{AbstractProductClient, WasmProductClient},
};

#[wasm_bindgen(typescript_custom_section)]
const _BUILDER_PARAMS: &str = r#"
import { Transaction } from "@iota/iota-sdk/transactions";

export interface OperationParams<O extends Operation> {
  operation: O,
  gasBudget?: bigint,
  gasPrice?: bigint,
  sponsor?: string,
  expiration?: bigint,
}

export interface OperationAndTransaction<O extends Operation> {
  operation: O,
  transaction: Transaction,
}

export interface GasStationOptions {
  url: string,
  gasReserveDuration?: bigint,
  headers?: Record<string, string>,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = OperationParams)]
    pub type WasmBuilderParams;

    #[wasm_bindgen(method, getter)]
    fn operation(this: &WasmBuilderParams) -> WasmOperation;

    #[wasm_bindgen(method, getter, js_name = gasBudget)]
    fn gas_budget(this: &WasmBuilderParams) -> Option<u64>;

    #[wasm_bindgen(method, getter, js_name = gasPrice)]
    fn gas_price(this: &WasmBuilderParams) -> Option<u64>;

    #[wasm_bindgen(method, getter)]
    fn sponsor(this: &WasmBuilderParams) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    fn expiration(this: &WasmBuilderParams) -> Option<u64>;

    #[wasm_bindgen(typescript_type = GasStationOptions)]
    #[derive(Clone)]
    pub type WasmGasStationOptions;

    #[wasm_bindgen(method, getter)]
    fn url(this: &WasmGasStationOptions) -> String;

    #[wasm_bindgen(method, getter, js_name = gasReserveDuration)]
    fn gas_reserve_duration(this: &WasmGasStationOptions) -> Option<u64>;

    #[wasm_bindgen(method, getter)]
    fn headers(this: &WasmGasStationOptions) -> Option<js_sys::Object>;
}

#[wasm_bindgen(js_name = OperationBuilder)]
pub struct WasmOperationBuilder(pub(crate) OperationBuilder<WasmOperation>);

#[wasm_bindgen(js_class = OperationBuilder)]
impl WasmOperationBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(params: &WasmBuilderParams) -> Result<Self, JsError> {
        let mut builder = OperationBuilder::new(params.operation());
        if let Some(budget) = params.gas_budget() {
            builder = builder.gas_budget(budget);
        }
        if let Some(price) = params.gas_price() {
            builder = builder.gas_price(price);
        }
        if let Some(sponsor_str) = params.sponsor() {
            builder = builder.sponsor(sponsor_str.parse()?);
        }
        if let Some(expiration) = params.expiration() {
            builder = builder.expiration(expiration);
        }

        Ok(Self(builder))
    }

    #[wasm_bindgen(unchecked_return_type = OperationAndTransaction)]
    pub async fn build(
        self,
        signer: &WasmTransactionSigner,
        client: &WasmProductClient,
    ) -> Result<WasmOperationAndTransaction, JsError> {
        let abstract_client = AbstractProductClient::try_from(client.clone())?;
        let (operation, tx) = self.0.build(signer, &abstract_client).await?;
        let wasm_tx = WasmTransaction::from_bcs_bytes(&tx.to_bcs())
            .map_err(|e| JsError::new(&ToString::to_string(&e.to_string())))?;

        Ok(WasmOperationAndTransaction {
            operation,
            transaction: wasm_tx,
        })
    }

    pub async fn execute(
        self,
        signer: &WasmTransactionSigner,
        client: &WasmProductClient,
    ) -> Result<WasmOperationOutput, JsError> {
        let abstract_client = AbstractProductClient::try_from(client.clone())?;
        self.0
            .execute(signer, &abstract_client)
            .await
            .map(WasmOperationOutput::from)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = executeWithSponsor)]
    pub async fn execute_with_sponsor(
        self,
        sender_signer: &WasmTransactionSigner,
        sponsor_signer: &WasmTransactionSigner,
        client: &WasmProductClient,
    ) -> Result<WasmOperationOutput, JsError> {
        let abstract_client = AbstractProductClient::try_from(client.clone())?;
        self.0
            .execute_with_sponsor(sender_signer, sponsor_signer, &abstract_client)
            .await
            .map(WasmOperationOutput::from)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = executeWithGasStation)]
    pub async fn execute_with_gas_station(
        self,
        gas_station_options: &WasmGasStationOptions,
        signer: &WasmTransactionSigner,
        client: &WasmProductClient,
    ) -> Result<WasmOperationOutput, JsError> {
        let gas_station_options = GasStationOptions::try_from(gas_station_options.clone())?;
        let abstract_client = AbstractProductClient::try_from(client.clone())?;
        self.0
            .execute_with_gas_station(gas_station_options, signer, &abstract_client)
            .await
            .map(WasmOperationOutput::from)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[wasm_bindgen(skip_typescript, js_name = OperationAndTransaction, getter_with_clone)]
pub struct WasmOperationAndTransaction {
    pub operation: WasmOperation,
    pub transaction: WasmTransaction,
}

#[wasm_bindgen(skip_typescript, js_name = OperationOutput, getter_with_clone)]
pub struct WasmOperationOutput {
    pub output: JsValue,
    #[wasm_bindgen(js_name = remainingEffects)]
    pub remaining_effects: JsValue,
}

impl From<OperationOutput<JsValue>> for WasmOperationOutput {
    fn from(value: OperationOutput<JsValue>) -> Self {
        WasmOperationOutput {
            output: value.output,
            remaining_effects: serde_wasm_bindgen::to_value(&value.remaining_effects)
                .expect("same repr"),
        }
    }
}

impl TryFrom<WasmGasStationOptions> for GasStationOptions {
    type Error = JsError;
    fn try_from(value: WasmGasStationOptions) -> Result<Self, Self::Error> {
        let url = value.url().parse()?;
        let gas_reserve_duration = value.gas_reserve_duration().map(Duration::from_secs);
        let headers = if let Some(headers) = value.headers() {
            let entry_map: HashMap<String, String> =
                serde_wasm_bindgen::from_value(headers.into())?;
            HeaderMap::try_from(&entry_map)?
        } else {
            HeaderMap::default()
        };

        let mut options = GasStationOptions::new(url);
        options.gas_reserve_duration = gas_reserve_duration;
        options.headers = headers;
        Ok(options)
    }
}
