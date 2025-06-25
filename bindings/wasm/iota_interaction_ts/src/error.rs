// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::fmt::{Debug};

use iota_interaction::types::execution_status::{
  CommandArgumentError, ExecutionFailureStatus, PackageUpgradeError, TypeArgumentError,
};
use serde::de::DeserializeOwned;
use thiserror::Error as ThisError;
use wasm_bindgen::JsValue;

use crate::common::into_sdk_type;
use crate::wasm_error::{stringify_js_error, WasmError, ErrorMessage, Result};

#[derive(ThisError, Debug, strum::IntoStaticStr)]
pub enum TsSdkError {
  #[error("[TsSdkError] PackageUpgradeError: {0}")]
  PackageUpgradeError(#[from] PackageUpgradeError),
  #[error("[TsSdkError] CommandArgumentError: {0}")]
  CommandArgumentError(#[from] CommandArgumentError),
  #[error("[TsSdkError] ExecutionFailureStatus: {0}")]
  ExecutionFailureStatus(#[from] ExecutionFailureStatus),
  #[error("[TsSdkError] TypeArgumentError: {0}")]
  TypeArgumentError(#[from] TypeArgumentError),
  #[error("[TsSdkError] AnyError: {0}")]
  AnyError(#[from] anyhow::Error),
  #[error("[TsSdkError] WasmError:{{\n   name: {0},\n   message: {1}\n}}")]
  WasmError(String, String),
  #[error("[TsSdkError] JsSysError: {0}")]
  JsSysError(String),
  #[error("[TsSdkError] TransactionSerializationError: {0}")]
  TransactionSerializationError(String),
  #[error("[TsSdkError] InvalidArgument: {0}")]
  InvalidArgument(String),
}

pub type TsSdkResult<T> = core::result::Result<T, TsSdkError>;

impl From<WasmError<'_>> for TsSdkError {
  fn from(err: WasmError<'_>) -> Self {
    TsSdkError::WasmError(err.name.to_string(), err.message.to_string())
  }
}

pub fn into_ts_sdk_result<T: DeserializeOwned>(result: Result<JsValue>) -> TsSdkResult<T> {
  let result_str = stringify_js_error(result);
  let js_value = result_str.map_err(TsSdkError::JsSysError)?;
  let ret_val: T = into_sdk_type(js_value)?;
  Ok(ret_val)
}

// This implementation is equivalent to `impl_wasm_error_from!(TsSdkError)`.
// We can't use the product_common::impl_wasm_error_from macro here because this would lead to
// a circular dependency.
impl From<TsSdkError> for WasmError<'_> {
  fn from(error: TsSdkError) -> Self {
    Self {
      message: Cow::Owned(ErrorMessage(&error).to_string()),
      name: Cow::Borrowed(error.into()),
    }
  }
}
