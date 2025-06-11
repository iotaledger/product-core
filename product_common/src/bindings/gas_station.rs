// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction_ts::core_client::WasmCoreClient;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError, JsValue};

use super::http_client::WasmHttpClient;
use super::transaction::WasmTransactionBuilder;
use crate::bindings::core_client::WasmManagedCoreClient;

const _GAS_STATION_OPTIONAL_PARAMS: &str = r#"
export interface GasStationParams {
  /** 
   * Duration of the gas reservation in seconds.
   * Defaults to 60.
  */
  gasReservationDuration?: bigint,
  /**
   * Bearer token to be included in the Authorization header in
   * all requests to the gas station.
  */
  bearerAuth?: string,
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = GasStationParams)]
  pub type WasmGasStationParams;
}

#[wasm_bindgen(js_class = TransactionBuilder)]
impl WasmTransactionBuilder {
  /// Execute this transaction using an IOTA Gas Station.
  #[wasm_bindgen(js_name = executeWithGasStation)]
  pub async fn execute_with_gas_station(
    self,
    client: &WasmCoreClient,
    gas_station_url: &str,
    http_client: &WasmHttpClient,
    options: Option<WasmGasStationParams>,
  ) -> Result<JsValue, JsValue> {
    let managed_client = WasmManagedCoreClient::from_wasm(client)?;
    let options = options
      .map(|wasm_options| serde_wasm_bindgen::from_value(wasm_options.into()))
      .transpose()?
      .unwrap_or_default();

    let apply_result = self
      .0
      .execute_with_gas_station(&managed_client, gas_station_url, http_client, options)
      .await
      .map_err(|e| JsError::from(e))?;

    Ok(apply_result)
  }
}
