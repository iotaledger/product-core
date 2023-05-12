// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::prelude::*;

use identity_iota::credential::vc_jwt_validation::CredentialValidationOptions as JwtCredentialValidationOptions;

/// Options to declare validation criteria when validating credentials.
#[wasm_bindgen(js_name = JwtCredentialValidationOptions)]
pub struct WasmJwtCredentialValidationOptions(pub(crate) JwtCredentialValidationOptions);

#[wasm_bindgen(js_class = JwtCredentialValidationOptions)]
impl WasmJwtCredentialValidationOptions {
  #[wasm_bindgen(constructor)]
  pub fn new(options: IJwtCredentialValidationOptions) -> Result<WasmJwtCredentialValidationOptions> {
    let options: JwtCredentialValidationOptions = options.into_serde().wasm_result()?;
    Ok(WasmJwtCredentialValidationOptions::from(options))
  }

  /// Creates a new `JwtCredentialValidationOptions` with defaults.
  #[allow(clippy::should_implement_trait)]
  #[wasm_bindgen]
  pub fn default() -> WasmJwtCredentialValidationOptions {
    WasmJwtCredentialValidationOptions::from(JwtCredentialValidationOptions::default())
  }
}

impl_wasm_json!(WasmJwtCredentialValidationOptions, JwtCredentialValidationOptions);
impl_wasm_clone!(WasmJwtCredentialValidationOptions, JwtCredentialValidationOptions);

impl From<JwtCredentialValidationOptions> for WasmJwtCredentialValidationOptions {
  fn from(options: JwtCredentialValidationOptions) -> Self {
    Self(options)
  }
}

impl From<WasmJwtCredentialValidationOptions> for JwtCredentialValidationOptions {
  fn from(options: WasmJwtCredentialValidationOptions) -> Self {
    options.0
  }
}

//Todo: add `StatusCheck` here if `CredentialValidationOptions` is be deleted.

// Interface to allow creating `JwtCredentialValidationOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwtCredentialValidationOptions")]
  pub type IJwtCredentialValidationOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWT_CREDENTIAL_VALIDATION_OPTIONS: &'static str = r#"
/** Holds options to create a new `JwtCredentialValidationOptions`. */
interface IJwtCredentialValidationOptions {
    /** Declare that the credential is **not** considered valid if it expires before this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly earliestExpiryDate?: Timestamp;

    /** Declare that the credential is **not** considered valid if it was issued later than this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly latestIssuanceDate?: Timestamp;

    /** Validation behaviour for `credentialStatus`.
     *
     * Default: `StatusCheck.Strict`. */
    readonly status?: StatusCheck;

    /** Options which affect the verification of the signature on the credential. */
    readonly verifierOptions?: VerifierOptions;

}"#;
