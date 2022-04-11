// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::StorageTestSuite;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::common::PromiseVoid;

use super::WasmStorage;

/// A test suite for the `Storage` interface.
///
/// This module contains a set of tests that a correct storage implementation
/// should pass. Note that not every edge case is tested.
///
/// Tests usually rely on multiple interface methods being implemented, so they should only
/// be run on a fully implemented version. That's why there is not a single test case for every
/// interface method.
#[wasm_bindgen(js_name = StorageTestSuite)]
pub struct WasmStorageTestSuite;

macro_rules! expose_to_wasm {
  ($test_name:ident, $js_name:ident) => {
    #[wasm_bindgen(js_class = StorageTestSuite)]
    impl WasmStorageTestSuite {
      #[wasm_bindgen(js_name = $js_name)]
      pub fn $test_name(storage: WasmStorage) -> PromiseVoid {
        let promise = future_to_promise(async move {
          StorageTestSuite::$test_name(storage)
            .await
            .map_err(|err| {
              let errors: Vec<String> = err.chain().map(|error| error.to_string()).collect();
              let output: String = AsRef::<[String]>::as_ref(&errors).join(": ");
              JsValue::from_str(&output)
            })
            .map(|_| JsValue::undefined())
        });

        promise.unchecked_into::<PromiseVoid>()
      }
    }
  };
}

expose_to_wasm!(did_create_generate_key_test, didCreateGenerateKeyTest);
expose_to_wasm!(did_create_private_key_test, didCreatePrivateKeyTest);
expose_to_wasm!(did_list_test, didListTest);
expose_to_wasm!(did_purge_test, didPurgeTest);
expose_to_wasm!(key_generate_test, keyGenerateTest);
expose_to_wasm!(key_delete_test, keyDeleteTest);
expose_to_wasm!(key_insert_test, keyInsertTest);
expose_to_wasm!(key_sign_ed25519_test, keySignEd25519Test);
