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
}
