// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction_ts::core_client::WasmCoreClient;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsError, JsValue};

use super::http_client::{WasmHeaderMap, WasmHttpClient};
use super::transaction::{WasmTransactionBuilder, WasmTransactionOutput};
use crate::bindings::core_client::WasmManagedCoreClient;
use crate::gas_station::{GasStationError, GasStationOptions};

#[wasm_bindgen(module = "@iota/iota_interaction_ts/gas-station")]
extern "C" {
  #[wasm_bindgen(typescript_type = GasStationParamsI)]
  pub type WasmGasStationParamsI;
}

#[wasm_bindgen(js_name = GasStationParams)]
pub struct WasmGasStationParams(pub(crate) GasStationOptions);

#[wasm_bindgen(js_class = GasStationParams)]
impl WasmGasStationParams {
  #[wasm_bindgen(constructor)]
  pub fn new(params: Option<WasmGasStationParamsI>) -> Result<Self, JsValue> {
    let params = params
      .map(|params| serde_wasm_bindgen::from_value(params.into()))
      .transpose()?
      .unwrap_or_default();
    Ok(Self(params))
  }

  /// Adds an `Authorization` header using `token` as a bearer token.
  #[wasm_bindgen(js_name = withAuthToken)]
  pub fn with_auth_token(self, token: &str) -> Self {
    Self(self.0.with_auth_token(token))
  }

  #[wasm_bindgen(getter, js_name = gasReservationDuration)]
  pub fn gas_reservation_duration(&self) -> u64 {
    self.0.gas_reservation_duration.as_secs()
  }

  #[wasm_bindgen(getter)]
  pub fn headers(&self) -> Result<WasmHeaderMap, JsValue> {
    let wasm_headers = serde_wasm_bindgen::to_value(&self.0.headers)?;
    Ok(wasm_headers.unchecked_into())
  }
}

#[wasm_bindgen(js_class = TransactionBuilder)]
impl WasmTransactionBuilder {
  /// Execute this transaction using an IOTA Gas Station.
  #[wasm_bindgen(js_name = executeWithGasStation, skip_typescript)]
  pub async fn execute_with_gas_station(
    self,
    client: &WasmCoreClient,
    gas_station_url: &str,
    http_client: &WasmHttpClient,
    options: Option<WasmGasStationParams>,
  ) -> Result<WasmTransactionOutput, JsValue> {
    let managed_client = WasmManagedCoreClient::from_wasm(client)?;
    let options = options
      .map(|wasm_options| serde_wasm_bindgen::from_value(wasm_options.into()))
      .transpose()?
      .unwrap_or_default();

    let tx_output = self
      .0
      .execute_with_gas_station(&managed_client, gas_station_url, http_client, options)
      .await?;

    Ok(tx_output.into())
  }
}

impl From<GasStationError> for JsValue {
  fn from(error: GasStationError) -> Self {
    let error = anyhow::Error::from(error);
    let js_error = JsError::new(&format!("{error:#}"));

    js_error.into()
  }
}
