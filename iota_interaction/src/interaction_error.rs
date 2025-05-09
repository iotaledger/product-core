// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur while using iota_interaction.

/// This type represents all possible errors that can occur while using iota_interaction.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// failed to connect to network.
  #[error("failed to connect to iota network node; {0:?}")]
  Network(String, #[source] crate::error::Error),
  // /// could not lookup an object ID.
  // #[error("failed to lookup an object; {0}")]
  // ObjectLookup(String),
  /// Caused by invalid or missing arguments.
  #[error("invalid or missing argument: {0}")]
  InvalidArgument(String),
  /// Caused by issues with paying for transaction.
  #[error("issue with gas for transaction: {0}")]
  GasIssue(String),
  /// Could not sign transaction.
  #[error("failed to sign transaction; {0}")]
  TransactionSigningFailed(String),
  /// Could not execute transaction.
  #[error("transaction execution failed; {0}")]
  TransactionExecutionFailed(#[from] crate::error::Error),
  /// Transaction yielded invalid response. This usually means that the transaction was executed but did not produce
  /// the expected result.
  #[error("transaction returned an unexpected response; {0}")]
  TransactionUnexpectedResponse(String),
  #[error("unexpected state when looking up a previous transaction; {0}")]
  /// Unexpected state when looking up the transaction history of an IOTA object.
  InvalidTransactionHistory(String),
  /// An error caused by either a connection issue or an invalid RPC call.
  #[error("RPC error: {0}")]
  RpcError(String),
  /// An error caused by a bcs serialization or deserialization.
  #[error("BCS error: {0}")]
  BcsError(#[from] bcs::Error),
}

/// Can be used for example like `map_err(interaction_err)` to convert other error
/// types to interaction_error::Error.
pub fn interaction_err<T>(error: T) -> Error
where
  Error: From<T>,
{
  error.into()
}
