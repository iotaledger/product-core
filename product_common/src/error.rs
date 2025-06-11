// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// This type represents errors that can occur when constructing credentials and presentations or their serializations.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// Caused by an invalid network name.
  #[error("\"{0}\" is not a valid network name in the context of the `iota` did method")]
  InvalidNetworkName(String),
  /// failed to connect to network.
  #[error("failed to connect to iota network node; {0:?}")]
  Network(String, #[source] iota_interaction::error::Error),
  /// Caused by issues with paying for transaction.
  #[error("issue with gas for transaction: {0}")]
  GasIssue(String),
  /// Could not build transaction.
  #[error("failed to build transaction; {0}")]
  TransactionBuildingFailed(String),
  /// Could not sign transaction.
  #[error("failed to sign transaction; {0}")]
  TransactionSigningFailed(String),
  /// A transaction was successfully executed on the ledger, but its off-chain logic couldn't be applied.
  #[error("failed to parse transaction effects: {source}")]
  TransactionOffChainApplicationFailure {
    /// The actual error coming from `apply`.
    #[source]
    source: Box<Self>,
    /// The raw RPC response, as received by the client.
    // Dev-comment: Neeeded to box this to avoid clippy complaining about the size of this variant.
    #[cfg(not(target_arch = "wasm32"))]
    response: Box<iota_sdk::rpc_types::IotaTransactionBlockResponse>,
    /// JSON-encoded string representation for the actual execution's RPC response.
    #[cfg(target_arch = "wasm32")]
    response: String,
  },
  /// Transaction yielded invalid response. This usually means that the transaction was executed but did not produce
  /// the expected result.
  #[error("transaction returned an unexpected response; {0}")]
  TransactionUnexpectedResponse(String),

  /// Transaction specific error.
  #[error("Transaction specific error: {0}")]
  Transaction(Box<dyn std::error::Error + Send + Sync + 'static>),

  /// Gas Station specific error.
  #[cfg(feature = "gas-station")]
  #[error("gas-station sponsoring failed: {0}")]
  GasStation(#[from] crate::gas_station::GasStationError),
}
