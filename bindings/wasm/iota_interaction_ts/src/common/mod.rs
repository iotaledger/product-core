// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod object;
pub mod types;
pub mod utils;

pub use types::*;
pub use utils::*;

// Copy of the console_log macro from product_common/src/bindings/macros.rs
// as we can't use the product_common crate here due to circular dependencies.
pub mod macros {
  use wasm_bindgen::prelude::wasm_bindgen;

  /// Log to console utility without the need for web_sys dependency
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn js_console_log(s: &str);
  }

  // Copy of the console_log macro from product_common/src/bindings/macros.rs
  // as we can't use the product_common crate here due to circular dependencies.
  /// Logging macro without the need for web_sys dependency
  macro_rules! console_log {
    ($($tt:tt)*) => {
      $crate::common::macros::js_console_log((format!($($tt)*)).as_str())
    }
  }

  pub(crate) use console_log;
}
