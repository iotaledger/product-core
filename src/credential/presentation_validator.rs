// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::WasmFailFast;
use crate::credential::WasmPresentation;
use crate::credential::WasmPresentationValidationOptions;
use crate::did::ArrayDocumentOrArrayResolvedDocument;
use crate::did::DocumentOrResolvedDocument;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::client::PresentationValidator;
use identity_iota::client::ResolvedIotaDocument;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = PresentationValidator, inspectable)]
pub struct WasmPresentationValidator;

#[wasm_bindgen(js_class = PresentationValidator)]
impl WasmPresentationValidator {
  /// Validate a `Presentation`.
  ///
  /// The following properties are validated according to `options`:
  /// - the semantic structure of the presentation,
  /// - the holder's signature,
  /// - the relationship between the holder and the credential subjects,
  /// - the signatures and some properties of the constituent credentials (see
  /// `CredentialValidator::validate`).
  ///
  /// ### Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the presentation can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// #### The state of the supplied DID Documents.
  /// The caller must ensure that the DID Documents in `holder` and `issuers` are up-to-date. The convenience methods
  /// `Resolver::resolve_presentation_holder` and `Resolver::resolve_presentation_issuers`
  /// can help extract the latest available states of these DID Documents.
  ///
  /// #### Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// ### Errors
  /// An error is returned whenever a validated condition is not satisfied.
  #[wasm_bindgen]
  pub fn validate(
    presentation: &WasmPresentation,
    holder: &DocumentOrResolvedDocument,
    issuers: &ArrayDocumentOrArrayResolvedDocument,
    options: &WasmPresentationValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<()> {
    let holder: ResolvedIotaDocument = holder.into_serde().wasm_result()?;
    let issuers: Vec<ResolvedIotaDocument> = issuers.into_serde().wasm_result()?;
    PresentationValidator::validate(&presentation.0, &holder, &issuers, &options.0, fail_fast.into()).wasm_result()
  }

  /// Verify the presentation's signature using the resolved document of the holder.
  ///
  /// ### Warning
  /// The caller must ensure that the DID Document of the holder is up-to-date.
  ///
  /// ### Errors
  /// Fails if the `holder` does not match the `presentation`'s holder property.
  /// Fails if signature verification against the holder document fails.
  #[wasm_bindgen(js_name = verifyPresentationSignature)]
  pub fn verify_presentation_signature(
    presentation: &WasmPresentation,
    holder: &DocumentOrResolvedDocument,
    options: &WasmVerifierOptions,
  ) -> Result<()> {
    let holder: ResolvedIotaDocument = holder.into_serde().wasm_result()?;
    PresentationValidator::verify_presentation_signature(&presentation.0, &holder, &options.0).wasm_result()
  }

  /// Validates the semantic structure of the `Presentation`.
  #[wasm_bindgen(js_name = checkStructure)]
  pub fn check_structure(presentation: &WasmPresentation) -> Result<()> {
    PresentationValidator::check_structure(&presentation.0).wasm_result()
  }
}
