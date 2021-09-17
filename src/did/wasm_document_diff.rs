// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::str::FromStr;

use identity::iota::{DocumentDiff, MessageId, TangleRef};
use wasm_bindgen::prelude::*;

use crate::did::{WasmDID, WasmDocument};
use crate::error::{Result, WasmResult};

/// Defines the difference between two DID [`Document`]s' JSON representations.
#[wasm_bindgen(js_name = DocumentDiff, inspectable)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WasmDocumentDiff(pub(crate) DocumentDiff);

#[wasm_bindgen(js_class = DocumentDiff)]
impl WasmDocumentDiff {
  /// Returns the DID of the associated DID Document.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDID {
    WasmDID::from(self.0.id().clone())
  }

  /// Returns the DID of the associated DID Document.
  #[wasm_bindgen(getter = did)]
  pub fn did(&self) -> WasmDID {
    self.id()
  }

  /// Returns the raw contents of the DID Document diff.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = diff)]
  pub fn diff(&self) -> String {
    self.0.diff().to_owned()
  }

  /// Returns the message_id of the DID Document diff.
  #[wasm_bindgen(getter = messageId)]
  pub fn message_id(&self) -> String {
    self.0.message_id().to_string()
  }

  /// Sets the message_id of the DID Document diff.
  #[wasm_bindgen(setter = messageId)]
  pub fn set_message_id(&mut self, message_id: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(message_id).wasm_result()?;
    self.0.set_message_id(message_id);
    Ok(())
  }

  /// Returns the Tangle message id of the previous DID Document diff.
  #[wasm_bindgen(getter = previousMessageId)]
  pub fn previous_message_id(&self) -> String {
    self.0.previous_message_id().to_string()
  }

  /// Sets the Tangle message id of the previous DID Document diff.
  #[wasm_bindgen(setter = previousMessageId)]
  pub fn set_previous_message_id(&mut self, message_id: &str) -> Result<()> {
    let previous_message_id: MessageId = MessageId::from_str(message_id).wasm_result()?;
    self.0.set_previous_message_id(previous_message_id);
    Ok(())
  }

  /// Returns a new DID Document which is the result of merging `self`
  /// with the given Document.
  pub fn merge(&self, document: &WasmDocument) -> Result<WasmDocument> {
    self.0.merge(&document.0).map(WasmDocument).wasm_result()
  }
}

impl From<DocumentDiff> for WasmDocumentDiff {
  fn from(document_diff: DocumentDiff) -> Self {
    Self(document_diff)
  }
}

impl Deref for WasmDocumentDiff {
  type Target = DocumentDiff;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
