// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "core-client")]
pub mod core_client;
#[cfg(feature = "transaction")]
pub mod transaction;

use iota_interaction_ts::error::WasmError;

impl From<crate::Error> for WasmError<'static> {
  fn from(e: crate::Error) -> Self {
    WasmError::new("product-common".into(), e.to_string().into())
  }
}
