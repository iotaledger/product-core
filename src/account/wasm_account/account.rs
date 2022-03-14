// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::AccountStorage;
use identity::account::PublishOptions;
use identity::account::Storage;
use identity::crypto::SetSignature;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifiableProperties;
use identity::iota::Client;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::types::WasmAutoSave;
use crate::common::PromiseVoid;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::WasmSignatureOptions;
use crate::did::PromiseResolvedDocument;
use crate::did::WasmDID;
use crate::did::WasmDocument;
use crate::did::WasmResolvedDocument;
use crate::error::Result;
use crate::error::WasmResult;

pub(crate) type AccountRc = Account<Rc<Client>>;

/// An account manages one identity.
///
/// It handles private keys, writing to storage and
/// publishing to the Tangle.
#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(pub(crate) Rc<RefCell<AccountRc>>);

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Returns the {@link DID} of the managed identity.
  #[wasm_bindgen(js_name = did)]
  pub fn did(&self) -> WasmDID {
    WasmDID::from(self.0.borrow().did().clone())
  }

  /// Returns whether auto-publish is enabled.
  #[wasm_bindgen]
  pub fn autopublish(&self) -> bool {
    self.0.borrow().autopublish()
  }

  /// Returns the auto-save configuration value.
  #[wasm_bindgen]
  pub fn autosave(&self) -> WasmAutoSave {
    WasmAutoSave(self.0.borrow().autosave())
  }

  /// Returns a copy of the document managed by the `Account`.
  #[wasm_bindgen]
  pub fn document(&self) -> WasmDocument {
    let document: IotaDocument = self.0.borrow().document().clone();
    WasmDocument::from(document)
  }

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  #[wasm_bindgen(js_name = resolveIdentity)]
  pub fn resolve_identity(&self) -> PromiseResolvedDocument {
    let account: Rc<RefCell<AccountRc>> = self.0.clone();

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow()
        .resolve_identity()
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });
    promise.unchecked_into::<PromiseResolvedDocument>()
  }

  /// Removes the identity from the local storage entirely.
  ///
  /// Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!
  #[wasm_bindgen(js_name = deleteIdentity)]
  pub fn delete_identity(self) -> PromiseVoid {
    // Get IotaDID and storage from the account.
    let did: IotaDID = self.0.borrow().did().to_owned();
    let storage: Arc<dyn Storage> = Arc::clone(self.0.borrow().storage());

    future_to_promise(async move {
      // Create a new account since `delete_identity` consumes it.
      let account: Result<AccountRc> = AccountBuilder::new()
        .storage(AccountStorage::Custom(storage))
        .load_identity(did)
        .await
        .wasm_result();

      match account {
        Ok(a) => a.delete_identity().await.wasm_result().map(|_| JsValue::undefined()),
        Err(e) => Err(e),
      }
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Push all unpublished changes to the tangle in a single message.
  #[wasm_bindgen]
  pub fn publish(&mut self, publish_options: Option<WasmPublishOptions>) -> PromiseVoid {
    let options: PublishOptions = publish_options.map(PublishOptions::from).unwrap_or_default();
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .publish_with_options(options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Signs a {@link Credential} with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedCredential)]
  pub fn create_signed_credential(
    &self,
    fragment: String,
    credential: &WasmCredential,
    signature_options: &WasmSignatureOptions,
  ) -> PromiseCredential {
    self
      .create_signed(fragment, credential.0.clone(), signature_options)
      .unchecked_into::<PromiseCredential>()
  }

  /// Signs a {@link Document} with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedDocument)]
  pub fn create_signed_document(
    &self,
    fragment: String,
    document: &WasmDocument,
    signature_options: &WasmSignatureOptions,
  ) -> PromiseDocument {
    self
      .create_signed(fragment, document.0.clone(), signature_options)
      .unchecked_into::<PromiseDocument>()
  }

  /// Signs a {@link Presentation} the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedPresentation)]
  pub fn create_signed_presentation(
    &self,
    fragment: String,
    presentation: &WasmPresentation,
    signature_options: &WasmSignatureOptions,
  ) -> PromisePresentation {
    self
      .create_signed(fragment, presentation.0.clone(), signature_options)
      .unchecked_into::<PromisePresentation>()
  }

  /// Signs arbitrary `data` with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedData)]
  pub fn create_signed_data(
    &self,
    fragment: String,
    data: &JsValue,
    signature_options: &WasmSignatureOptions,
  ) -> Result<PromiseVoid> {
    let verifiable_properties: VerifiableProperties = data.into_serde().wasm_result()?;
    Ok(self.create_signed(fragment, verifiable_properties, signature_options))
  }

  fn create_signed<U>(&self, fragment: String, mut data: U, signature_options: &WasmSignatureOptions) -> PromiseVoid
  where
    U: serde::Serialize + SetSignature + 'static,
  {
    let account = self.0.clone();
    let options: SignatureOptions = signature_options.0.clone();

    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut data, options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()?;
      JsValue::from_serde(&data).wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Overwrites the {@link Document} this account manages, **without doing any validation**.
  ///
  /// ### WARNING
  ///
  /// This method is dangerous and can easily corrupt the internal state,
  /// potentially making the identity unusable. Only call this if you fully
  /// understand the implications!
  #[wasm_bindgen(js_name = updateDocumentUnchecked)]
  pub fn update_document_unchecked(&mut self, document: &WasmDocument) -> PromiseVoid {
    let account = self.0.clone();
    let document_copy: IotaDocument = document.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .update_document_unchecked(document_copy)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Fetches the latest changes from the tangle and **overwrites** the local document.
  ///
  /// If a DID is managed from distributed accounts, this should be called before making changes
  /// to the identity, to avoid publishing updates that would be ignored.
  #[wasm_bindgen(js_name = fetchState)]
  pub fn fetch_state(&mut self) -> PromiseVoid {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .fetch_state()
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }
}

impl From<AccountRc> for WasmAccount {
  fn from(account: AccountRc) -> WasmAccount {
    WasmAccount(Rc::new(RefCell::new(account)))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Credential>")]
  pub type PromiseCredential;

  #[wasm_bindgen(typescript_type = "Promise<Presentation>")]
  pub type PromisePresentation;

  #[wasm_bindgen(typescript_type = "Promise<Document>")]
  pub type PromiseDocument;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "PublishOptions")]
  pub type WasmPublishOptions;

  #[wasm_bindgen(getter, method)]
  pub fn forceIntegrationUpdate(this: &WasmPublishOptions) -> Option<bool>;

  #[wasm_bindgen(getter, method)]
  pub fn signWith(this: &WasmPublishOptions) -> Option<String>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_PUBLISH_OPTIONS: &'static str = r#"
/**
 * Options to customize how identities are published to the Tangle.
**/
export type PublishOptions = {
    /**
     * Whether to force the publication to be an integration update.
     * If this option is not set, the account automatically determines whether
     * an update needs to be published as an integration or a diff update.
     * Publishing as an integration update is always valid, but not recommended
     * for identities with many updates.
     *
     * See the IOTA DID method specification for more details.
     */
     forceIntegrationUpdate?: boolean,


    /**
     * Set the fragment of a verification method with which to sign the update.
     * This must point to an Ed25519 method with a capability invocation
     * verification relationship.
     *
     *  If omitted, the default signing method on the Document will be used.
     */
     signWith?: string
 }
"#;

impl From<WasmPublishOptions> for PublishOptions {
  fn from(publish_options: WasmPublishOptions) -> Self {
    let mut options: PublishOptions = PublishOptions::new();

    if let Some(force_integration) = publish_options.forceIntegrationUpdate() {
      options = options.force_integration_update(force_integration);
    }

    if let Some(sign_with) = publish_options.signWith() {
      options = options.sign_with(sign_with);
    };
    options
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Account>")]
  pub type PromiseAccount;
}
