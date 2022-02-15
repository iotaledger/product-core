// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Account;
use identity::account::CreateServiceBuilder;
use identity::account::IdentityUpdater;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::Object;
use identity::core::Url;

use crate::account::wasm_account::WasmAccount;

use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::JsCast;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new Service to the DID Document.
  #[wasm_bindgen(js_name = createService)]
  pub fn create_service(&mut self, options: &CreateServiceOptions) -> Result<PromiseVoid> {
    let service_type: String = options.type_().ok_or(MissingRequiredField("type")).wasm_result()?;

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;
    let endpoint: String = options
      .endpoint()
      .ok_or(MissingRequiredField("endpoint"))
      .wasm_result()?;
    let endpoint: Url = Url::parse(endpoint.as_str()).wasm_result()?;
    let properties: Option<Object> = options.properties().into_serde().wasm_result()?;

    let account: Rc<RefCell<Account>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<Account> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_> = account.update_identity();
      let mut create_service: CreateServiceBuilder<'_> = updater
        .create_service()
        .fragment(fragment)
        .type_(service_type)
        .endpoint(endpoint);

      if let Some(properties) = properties {
        create_service = create_service.properties(properties)
      }

      create_service.apply().await.wasm_result().map(|_| JsValue::undefined())
    });

    Ok(promise.unchecked_into::<PromiseVoid>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CreateServiceOptions")]
  pub type CreateServiceOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(getter, method, js_name= type)]
  pub fn type_(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn endpoint(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn properties(this: &CreateServiceOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_CREATE_SERVICE_OPTIONS: &'static str = r#"
/**
 * Options for creating a new service on an identity.
 */
export type CreateServiceOptions = {
    /**
     * The identifier of the service in the document.
     */
    fragment: string,

    /**
     * The type of the service.
     */
    type: string,

    /**
     * The `ServiceEndpoint` of the service.
     */
    endpoint: string,

    /**
     * Additional properties of the service.
     */
    properties?: any,
  };
"#;
