// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "bindings", target_arch = "wasm32"))]
pub mod bindings;
#[cfg(feature = "core-client")]
pub mod core_client;
pub mod error;
#[cfg(feature = "gas-station")]
pub mod gas_station;
#[cfg(feature = "http-client")]
pub mod http_client;
pub mod network_name;
pub mod object;
pub mod package_registry;
pub mod well_known_networks;

#[cfg(feature = "transaction")]
pub(crate) mod iota_interaction_adapter;
#[cfg(feature = "transaction")]
pub mod transaction;

#[cfg(feature = "test-utils")]
pub mod test_utils;

pub use error::*;
