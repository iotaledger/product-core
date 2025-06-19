// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "core-client")]
pub mod core_client;
#[cfg(feature = "gas-station")]
pub mod gas_station;
#[cfg(feature = "http-client")]
pub mod http_client;
#[cfg(feature = "transaction")]
pub mod transaction;
#[cfg(feature = "binding-utils")]
pub mod utils;
pub mod macros;

use iota_interaction_ts::error::WasmError;

pub type WasmIotaAddress = String;
pub type WasmObjectID = String;

impl From<crate::Error> for WasmError<'static> {
  fn from(e: crate::Error) -> Self {
    WasmError::new("product-common".into(), e.to_string().into())
  }
}
