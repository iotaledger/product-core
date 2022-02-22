// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::AutoSave;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = AutoSave)]
pub struct WasmAutoSave(pub(crate) AutoSave);

/// Available auto-save behaviours.
#[wasm_bindgen(js_class = AutoSave)]
impl WasmAutoSave {
  /// Never save.
  #[wasm_bindgen]
  pub fn never() -> WasmAutoSave {
    Self(AutoSave::Never)
  }

  /// Save after every action.
  #[wasm_bindgen]
  pub fn every() -> WasmAutoSave {
    Self(AutoSave::Every)
  }

  /// Save after every N actions.
  #[wasm_bindgen]
  pub fn batch(number_of_actions: usize) -> WasmAutoSave {
    Self(AutoSave::Batch(number_of_actions))
  }
}
