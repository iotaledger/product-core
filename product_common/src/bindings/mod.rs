// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "core-client")]
pub mod core_client;
#[cfg(feature = "gas-station")]
pub mod gas_station;
#[cfg(feature = "http-client")]
pub mod http_client;
pub mod macros;
#[cfg(feature = "transaction")]
pub mod transaction;
#[cfg(feature = "binding-utils")]
pub mod utils;

pub use iota_interaction_ts::wasm_error;

pub type WasmIotaAddress = String;
pub type WasmObjectID = String;
