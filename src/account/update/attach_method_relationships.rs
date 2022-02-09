// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::cell::RefCell;
use std::cell::RefMut;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Account;
use identity::account::AttachMethodRelationshipBuilder;
use identity::account::IdentityUpdater;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany;

use identity::did::MethodRelationship;

use crate::account::wasm_account::WasmAccount;
use crate::account::wasm_method_relationship::WasmMethodRelationship;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Attach one or more verification relationships to a method.
  ///
  /// Note: the method must exist and be in the set of verification methods;
  /// it cannot be an embedded method.
  #[wasm_bindgen(js_name = attachMethodRelationships)]
  pub fn attach_method_relationships(&mut self, options: &AttachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<MethodRelationship> = options
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?
      .into_iter()
      .map(MethodRelationship::from)
      .collect();

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let account: Rc<RefCell<Account>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      if relationships.is_empty() {
        return Ok(JsValue::undefined());
      }
      let mut account: RefMut<Account> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_> = account.update_identity();
      let mut attach_relationship: AttachMethodRelationshipBuilder<'_> =
        updater.attach_method_relationship().fragment(fragment);

      for relationship in relationships {
        attach_relationship = attach_relationship.relationship(relationship);
      }

      attach_relationship
        .apply()
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "AttachMethodRelationshipOptions")]
  pub type AttachMethodRelationshipOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &AttachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn relationships(this: &AttachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_ATTACH_METHOD_RELATIONSHIP_OPTIONS: &'static str = r#"
/**
 * Options for attaching one or more verification relationships to a method on an identity.
 */
export type AttachMethodRelationshipOptions = {
    /**
     * The identifier of the method in the document.
     */
    fragment: string,

    /**
     * The relationships to add;
     */
    relationships: MethodRelationship | MethodRelationship[]
};
"#;
