// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::WasmCoreDID;
use crate::did::WasmCoreDocument;
use crate::error::WasmError;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDID;
use crate::stardust::WasmStardustDocument;
use identity_iota::credential::AbstractValidatorDocument;
use identity_iota::did::CoreDID;
use identity_iota::did::CoreDocument;
use identity_iota::did::DID;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize)]
#[serde(untagged)]
/// Temporary type used to convert to and from Box<dyn ValidatorDocument> until
/// we port the Document trait to these bindings.
pub enum RustSupportedDocument {
  Stardust(StardustDocument),
  Core(CoreDocument),
}

impl From<RustSupportedDocument> for JsValue {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(doc) => JsValue::from(WasmCoreDocument::from(doc)),
      RustSupportedDocument::Stardust(doc) => JsValue::from(WasmStardustDocument(doc)),
    }
  }
}

impl From<RustSupportedDocument> for AbstractValidatorDocument {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(core_doc) => AbstractValidatorDocument::from(core_doc),
      RustSupportedDocument::Stardust(stardust_doc) => AbstractValidatorDocument::from(stardust_doc),
    }
  }
}

impl TryFrom<AbstractValidatorDocument> for RustSupportedDocument {
  type Error = WasmError<'static>;
  fn try_from(value: AbstractValidatorDocument) -> std::result::Result<Self, Self::Error> {
    let upcast = value.into_any();
    let supported_document = match upcast.downcast::<CoreDocument>() {
      Ok(doc) => RustSupportedDocument::Core(*doc),
      Err(retry) => {
        if let Ok(doc) = retry.downcast::<StardustDocument>() {
          RustSupportedDocument::Stardust(*doc)
        } else {
          Err(WasmError::new(
            "CastingError".into(),
            "Failed to cast the resolved did output to the required document type".into(),
          ))?
        }
      }
    };
    Ok(supported_document)
  }
}

impl TryFrom<CoreDID> for SupportedDID {
  type Error = JsValue;

  fn try_from(did: CoreDID) -> Result<Self, Self::Error> {
    let js: JsValue = if did.method() == StardustDID::METHOD {
      let ret: StardustDID = StardustDID::try_from_core(did).wasm_result()?;
      JsValue::from(WasmStardustDID::from(ret))
    } else {
      JsValue::from(WasmCoreDID::from(did))
    };

    Ok(js.unchecked_into::<SupportedDID>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Array<StardustDocument | CoreDocument>>")]
  pub type PromiseArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Promise<StardustDocument | CoreDocument>")]
  pub type PromiseSupportedDocument;

  #[wasm_bindgen(typescript_type = "StardustDocument | CoreDocument")]
  pub type SupportedDocument;

  #[wasm_bindgen(typescript_type = "StardustDocument | CoreDocument | undefined")]
  pub type OptionSupportedDocument;

  #[wasm_bindgen(typescript_type = "Array<StardustDocument | CoreDocument>")]
  pub type ArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Array<StardustDocument | CoreDocument> | undefined")]
  pub type OptionArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "CoreDID | StardustDID")]
  pub type SupportedDID;
}
