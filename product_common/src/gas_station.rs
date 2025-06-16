// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::error;
use std::fmt::{Debug, Display};
use std::time::Duration;

use fastcrypto::encoding::{Base64, Encoding as _};
use fastcrypto::traits::EncodeDecodeBase64 as _;
use iota_interaction::rpc_types::{IotaObjectRef, IotaTransactionBlockEffects};
use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::crypto::Signature;
use iota_interaction::types::transaction::TransactionData;
use serde::{Deserialize, Serialize};

use crate::http_client::{HttpClient, Method, Request, Url, UrlParsingError};
use crate::Error;

pub(crate) const DEFAULT_GAS_RESERVATION_DURATION: u64 = 60; // 1 minute.
pub(crate) const DEFAULT_GAS_BUDGET_RESERVATION: u64 = 1_000_000_000; // 1 IOTA.

fn default_gas_reservation() -> Duration {
  Duration::from_secs(DEFAULT_GAS_RESERVATION_DURATION)
}

/// Possible types of error that might occur when executing
/// transactions through an IOTA Gas Station.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
  Url(UrlParsingError),
  GasReservation(GasStationRequestError),
  TxExecution(GasStationRequestError),
  TxDataBuilding(Box<Error>),
  TxApplication(Box<Error>),
}

/// Failure for the execution of a transaction through an IOTA Gas Station.
#[derive(Debug)]
#[non_exhaustive]
pub struct GasStationError {
  pub kind: ErrorKind,
}

impl GasStationError {
  #[inline(always)]
  pub(crate) fn new(kind: ErrorKind) -> Self {
    Self { kind }
  }

  #[inline(always)]
  pub(crate) fn from_reservation_error(e: GasStationRequestError) -> Self {
    Self {
      kind: ErrorKind::GasReservation(e),
    }
  }

  #[inline(always)]
  pub(crate) fn from_tx_execution_error(e: GasStationRequestError) -> Self {
    Self {
      kind: ErrorKind::TxExecution(e),
    }
  }
}

impl Display for GasStationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "failed to execute transaction with gas station")
  }
}

impl error::Error for GasStationError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    use ErrorKind::*;
    match &self.kind {
      Url(e) => Some(e),
      GasReservation(e) => Some(e),
      TxExecution(e) => Some(e),
      TxDataBuilding(e) => Some(e),
      TxApplication(e) => Some(e),
    }
  }
}

/// Optional configuration to be passed to the gas-station when sponsoring a transaction.
#[non_exhaustive]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasStationOptions {
  /// Duration of the gas allocation. Default value: `60` seconds.
  #[serde(default = "default_gas_reservation")]
  pub gas_reservation_duration: Duration,
  /// Bearer token to be included in all requests' "Authentication" header.
  pub bearer_auth: Option<String>,
}

impl Default for GasStationOptions {
  fn default() -> Self {
    Self {
      gas_reservation_duration: default_gas_reservation(),
      bearer_auth: None,
    }
  }
}

impl Debug for GasStationOptions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GasStationOptions")
      .field("gas_reservation_duration", &self.gas_reservation_duration)
      .field("bearer_auth", &self.bearer_auth.as_deref().and(Some("[REDACTED]")))
      .finish_non_exhaustive()
  }
}

#[derive(Debug, Serialize)]
struct ReserveGasRequest {
  gas_budget: u64,
  reserve_duration_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReserveGasResponse {
  result: Option<ReserveGasResult>,
  error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReserveGasResult {
  pub(crate) sponsor_address: IotaAddress,
  pub(crate) reservation_id: u64,
  pub(crate) gas_coins: Vec<IotaObjectRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GasStationRequestErrorKind {
  BodySerialization,
  #[non_exhaustive]
  HttpClient {
    method: Method,
    url: Url,
  },
  BodyDeserialization,
  #[non_exhaustive]
  InvalidResponse {
    message: Option<String>,
  },
}

impl GasStationRequestErrorKind {
  fn to_error_message(&self) -> Cow<'static, str> {
    use GasStationRequestErrorKind::*;
    match self {
      BodySerialization => "failed to serialize request's body".into(),
      HttpClient { method, url } => format!("HTTP request `{method} {url}` failed").into(),
      BodyDeserialization => "failed to deserialize respose's body".into(),
      InvalidResponse { message, .. } => {
        let msg = Cow::Borrowed("invalid response");
        let Some(response_error) = message else {
          return msg;
        };

        format!("{msg}: {response_error}").into()
      }
    }
  }
}

impl Display for GasStationRequestErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_error_message())
  }
}

/// Gas station request error.
#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct GasStationRequestError {
  kind: GasStationRequestErrorKind,
  source: Option<Box<dyn error::Error + Send + Sync>>,
}

impl GasStationRequestError {
  fn new(kind: GasStationRequestErrorKind) -> Self {
    Self { kind, source: None }
  }

  fn with_source(mut self, e: impl Into<Box<dyn error::Error + Send + Sync>>) -> Self {
    self.source = Some(e.into());
    self
  }

  /// Returns a reference to this error's [ErrorKind].
  pub fn kind(&self) -> &GasStationRequestErrorKind {
    &self.kind
  }
}

pub(crate) async fn reserve_gas<H>(
  gas_station_url: &Url,
  gas_budget: u64,
  reserve_duration_secs: u64,
  auth_token: Option<&str>,
  http_client: &H,
) -> Result<ReserveGasResult, GasStationRequestError>
where
  H: HttpClient,
  H::Error: Into<Box<dyn error::Error + Send + Sync>>,
{
  // Prepare the request.
  let url = gas_station_url
    .join("/v1/reserve_gas")
    .expect("a valid URL joined by another valid path is valid");
  let headers = auth_token
    .into_iter()
    .map(|token| ("Authorization".to_owned(), format!("Bearer {token}")))
    .collect();
  let body = serde_json::to_vec(&ReserveGasRequest {
    gas_budget,
    reserve_duration_secs,
  })
  .map_err(|e| GasStationRequestError::new(GasStationRequestErrorKind::BodySerialization).with_source(e))?;

  let reserve_gas_req = Request {
    method: Method::Post,
    url: url.clone(),
    headers,
    payload: body,
  };

  let response = http_client.send(reserve_gas_req).await.map_err(|e| {
    GasStationRequestError::new(GasStationRequestErrorKind::HttpClient {
      method: Method::Post,
      url,
    })
    .with_source(e)
  })?;

  let ReserveGasResponse { result, error } = serde_json::from_slice(&response.payload)
    .map_err(|e| GasStationRequestError::new(GasStationRequestErrorKind::BodyDeserialization).with_source(e))?;

  let Some(reservation_result) = result else {
    return Err(GasStationRequestError::new(
      GasStationRequestErrorKind::InvalidResponse { message: error },
    ));
  };

  Ok(reservation_result)
}

#[derive(Debug, Serialize)]
struct ExecuteTxRequest {
  reservation_id: u64,
  tx_bytes: String,
  user_sig: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExecuteTxResponse {
  effects: Option<IotaTransactionBlockEffects>,
  error: Option<String>,
}

pub(crate) async fn execute_sponsored_tx<H>(
  gas_station_url: &Url,
  tx_data: TransactionData,
  sender_sig: Signature,
  reservation_id: u64,
  auth_token: Option<&str>,
  http_client: &H,
) -> Result<IotaTransactionBlockEffects, GasStationRequestError>
where
  H: HttpClient,
  H::Error: Into<Box<dyn error::Error + Send + Sync>>,
{
  // Prepare the request.
  let url = gas_station_url
    .join("/v1/execute_tx")
    .expect("a valid URL joined by another valid path is valid");
  let headers = auth_token
    .into_iter()
    .map(|token| ("Authorization".to_owned(), format!("Bearer {token}")))
    .collect();

  let tx_bcs = bcs::to_bytes(&tx_data)
    .map_err(|e| GasStationRequestError::new(GasStationRequestErrorKind::BodySerialization).with_source(e))?;
  let body = serde_json::to_vec(&ExecuteTxRequest {
    reservation_id,
    tx_bytes: Base64::encode(&tx_bcs),
    user_sig: sender_sig.encode_base64(),
  })
  .map_err(|e| GasStationRequestError::new(GasStationRequestErrorKind::BodySerialization).with_source(e))?;
  let execute_tx_req = Request {
    method: Method::Post,
    url: url.clone(),
    headers,
    payload: body,
  };

  let response = http_client.send(execute_tx_req).await.map_err(|e| {
    GasStationRequestError::new(GasStationRequestErrorKind::HttpClient {
      method: Method::Post,
      url,
    })
    .with_source(e)
  })?;

  let ExecuteTxResponse { effects, error } = serde_json::from_slice(&response.payload)
    .map_err(|e| GasStationRequestError::new(GasStationRequestErrorKind::BodyDeserialization).with_source(e))?;

  let Some(effects) = effects else {
    return Err(GasStationRequestError::new(
      GasStationRequestErrorKind::InvalidResponse { message: error },
    ));
  };

  Ok(effects)
}
