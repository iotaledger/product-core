// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "bindings", target_arch = "wasm32"))]
pub mod bindings;
#[cfg(feature = "core-client")]
pub mod core_client;
pub mod error;
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
#[cfg(feature = "wasm-utils")]
pub mod wasm_utils;

pub use error::*;
