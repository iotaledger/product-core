// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error;
use std::fmt::{Debug, Display};
use std::time::Duration;

use fastcrypto::encoding::{Base64, Encoding as _};
use fastcrypto::traits::EncodeDecodeBase64 as _;
use iota_interaction::rpc_types::{IotaObjectRef, IotaTransactionBlockEffects};
use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::crypto::Signature;
use iota_interaction::types::transaction::TransactionData;
use serde::{Deserialize, Deserializer, Serialize};

use crate::http_client::{HeaderMap, HttpClient, Method, Request, Url, UrlParsingError};

pub(crate) const DEFAULT_GAS_RESERVATION_DURATION: u64 = 60; // 1 minute.
pub(crate) const DEFAULT_GAS_BUDGET_RESERVATION: u64 = 1_000_000_000; // 1 IOTA.
const WAIT_FOR_LOCAL_EXECUTION: &str = "waitForLocalExecution";

const fn default_gas_reservation() -> Duration {
  Duration::from_secs(DEFAULT_GAS_RESERVATION_DURATION)
}

/// Possible types of error that might occur when executing
/// transactions through an IOTA Gas Station.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
  /// Failed to parse the received string gas station URL
  /// as a valid URL.
  Url(UrlParsingError),
  /// A request to the gas-station failed.
  GasStationRequest(Box<GasStationRequestError>),
  // TODO: after refactoring product-core's error handling, change this opaque type.
  /// Failed to build transaction.
  TxDataBuilding(Box<dyn std::error::Error + Send + Sync>),
  /// Transaction was executed successfully but its effects couldn't be applied off-chain.
  TxApplication(Box<dyn std::error::Error + Send + Sync>),
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
}

impl From<GasStationRequestError> for GasStationError {
  fn from(value: GasStationRequestError) -> Self {
    Self {
      kind: ErrorKind::GasStationRequest(Box::new(value)),
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
      GasStationRequest(e) => Some(e.as_ref()),
      TxDataBuilding(e) => Some(e.as_ref()),
      TxApplication(e) => Some(e.as_ref()),
    }
  }
}

/// Optional configuration to be passed to the gas-station when sponsoring a transaction.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasStationOptions {
  /// Duration of the gas allocation. Default value: `60` seconds.
  #[serde(default = "default_gas_reservation", deserialize_with = "deserialize_duration_secs")]
  pub gas_reservation_duration: Duration,
  /// Headers to be included in all requests to the gas station.
  pub headers: HeaderMap,
}

fn deserialize_duration_secs<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
  u64::deserialize(deserializer).map(Duration::from_secs)
}

impl Default for GasStationOptions {
  fn default() -> Self {
    let mut headers = HeaderMap::default();
    headers.insert("Content-Type".to_owned(), vec!["application/json".to_owned()]);
    Self {
      gas_reservation_duration: default_gas_reservation(),
      headers,
    }
  }
}

impl GasStationOptions {
  /// Uses the given token to authenticate all gas station requests by including it
  /// in [Self::headers] as `("Authorization", "Bearer <auth_token>")`.
  pub fn with_auth_token(mut self, auth_token: impl AsRef<str>) -> Self {
    let value = format!("Bearer {}", auth_token.as_ref());
    self.headers.entry("Authorization".to_owned()).or_default().push(value);

    self
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

/// Possible failures of a gas station request.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
enum GasStationRequestErrorKind {
  /// Failed to serialize request's body.
  #[error("failed to serialize request's body")]
  #[non_exhaustive]
  BodySerialization {
    source: Box<dyn error::Error + Send + Sync>,
  },
  /// HTTP error.
  #[error("HTTP request `{method} {url}` failed")]
  #[non_exhaustive]
  HttpClient {
    method: Method,
    url: Url,
    source: Box<dyn error::Error + Send + Sync>,
  },
  /// Failed to deserialize request's body.
  #[error("failed to deserialize response's body")]
  #[non_exhaustive]
  BodyDeserialization { source: serde_json::Error },
  /// Invalid or unsuccessful response.
  #[error(
    "received an invalid response with status code `{status_code}`{}",
    .message.as_deref().map(|msg| format!(" and message \"{msg}\"")).unwrap_or_default()
  )]
  #[non_exhaustive]
  InvalidResponse { message: Option<String>, status_code: u16 },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GasStationRequestKind {
  #[non_exhaustive]
  ReserveGas,
  #[non_exhaustive]
  ExecuteTx,
}

impl GasStationRequestKind {
  const fn as_path(&self) -> &str {
    match self {
      Self::ReserveGas => "/v1/reserve_gas",
      Self::ExecuteTx => "/v1/execute_tx",
    }
  }
}

/// Gas station request error.
#[derive(Debug, thiserror::Error)]
#[error("request `{}` to gas station `{gas_station_url}` failed", .request_kind.as_path())]
pub struct GasStationRequestError {
  #[source]
  kind: GasStationRequestErrorKind,
  pub request_kind: GasStationRequestKind,
  pub gas_station_url: Url,
}

impl GasStationRequestError {
  fn new_reservation(kind: GasStationRequestErrorKind, gas_station_url: Url) -> Self {
    Self {
      kind,
      request_kind: GasStationRequestKind::ReserveGas,
      gas_station_url,
    }
  }

  fn new_execution(kind: GasStationRequestErrorKind, gas_station_url: Url) -> Self {
    Self {
      kind,
      request_kind: GasStationRequestKind::ExecuteTx,
      gas_station_url,
    }
  }
}

pub(crate) async fn reserve_gas<H>(
  gas_station_url: &Url,
  gas_budget: u64,
  reserve_duration_secs: u64,
  headers: &HeaderMap,
  http_client: &H,
) -> Result<ReserveGasResult, GasStationRequestError>
where
  H: HttpClient,
  H::Error: Into<Box<dyn error::Error + Send + Sync>>,
{
  // Prepare the request.
  let url = gas_station_url
    .join(GasStationRequestKind::ReserveGas.as_path())
    .expect("a valid URL joined by another valid path is valid");
  let body = serde_json::to_vec(&ReserveGasRequest {
    gas_budget,
    reserve_duration_secs,
  })
  .map_err(|e| {
    GasStationRequestError::new_reservation(
      GasStationRequestErrorKind::BodySerialization { source: e.into() },
      gas_station_url.clone(),
    )
  })?;

  let reserve_gas_req = Request {
    method: Method::Post,
    url: url.clone(),
    headers: headers.clone(),
    payload: body,
  };

  let response = http_client.send(reserve_gas_req).await.map_err(|e| {
    GasStationRequestError::new_reservation(
      GasStationRequestErrorKind::HttpClient {
        method: Method::Post,
        url,
        source: e.into(),
      },
      gas_station_url.clone(),
    )
  })?;
  let status_code = response.status_code;
  if status_code >= 400 {
    let is_text_body = response
      .headers
      .get("content-type")
      .is_some_and(|types| types.iter().any(|type_| type_ == "text/plain; charset=utf-8"));
    let maybe_err_msg = is_text_body.then(|| String::from_utf8(response.payload).ok()).flatten();

    return Err(GasStationRequestError::new_reservation(
      GasStationRequestErrorKind::InvalidResponse {
        message: maybe_err_msg,
        status_code,
      },
      gas_station_url.clone(),
    ));
  }

  let ReserveGasResponse { result, error } = serde_json::from_slice(&response.payload).map_err(|e| {
    GasStationRequestError::new_reservation(
      GasStationRequestErrorKind::BodyDeserialization { source: e },
      gas_station_url.clone(),
    )
  })?;

  let Some(reservation_result) = result else {
    return Err(GasStationRequestError::new_reservation(
      GasStationRequestErrorKind::InvalidResponse {
        message: error,
        status_code,
      },
      gas_station_url.clone(),
    ));
  };

  Ok(reservation_result)
}

#[derive(Debug, Serialize)]
struct ExecuteTxRequest {
  reservation_id: u64,
  tx_bytes: String,
  user_sig: String,
  request_type: String,
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
  headers: HeaderMap,
  http_client: &H,
) -> Result<IotaTransactionBlockEffects, GasStationRequestError>
where
  H: HttpClient,
  H::Error: Into<Box<dyn error::Error + Send + Sync>>,
{
  // Prepare the request.
  let url = gas_station_url
    .join(GasStationRequestKind::ExecuteTx.as_path())
    .expect("a valid URL joined by another valid path is valid");

  let tx_bcs = bcs::to_bytes(&tx_data).map_err(|e| {
    GasStationRequestError::new_execution(
      GasStationRequestErrorKind::BodySerialization { source: e.into() },
      gas_station_url.clone(),
    )
  })?;
  let body = serde_json::to_vec(&ExecuteTxRequest {
    reservation_id,
    tx_bytes: Base64::encode(&tx_bcs),
    user_sig: sender_sig.encode_base64(),
    request_type: WAIT_FOR_LOCAL_EXECUTION.to_owned(),
  })
  .map_err(|e| {
    GasStationRequestError::new_execution(
      GasStationRequestErrorKind::BodySerialization { source: e.into() },
      gas_station_url.clone(),
    )
  })?;
  let execute_tx_req = Request {
    method: Method::Post,
    url: url.clone(),
    headers,
    payload: body,
  };

  let response = http_client.send(execute_tx_req).await.map_err(|e| {
    GasStationRequestError::new_execution(
      GasStationRequestErrorKind::HttpClient {
        method: Method::Post,
        url,
        source: e.into(),
      },
      gas_station_url.clone(),
    )
  })?;
  let status_code = response.status_code;
  if status_code >= 400 {
    let is_text_body = response
      .headers
      .get("content-type")
      .is_some_and(|types| types.iter().any(|type_| type_ == "text/plain; charset=utf-8"));
    let maybe_err_msg = is_text_body.then(|| String::from_utf8(response.payload).ok()).flatten();

    return Err(GasStationRequestError::new_execution(
      GasStationRequestErrorKind::InvalidResponse {
        message: maybe_err_msg,
        status_code,
      },
      gas_station_url.clone(),
    ));
  }

  let ExecuteTxResponse { effects, error } = serde_json::from_slice(&response.payload).map_err(|e| {
    GasStationRequestError::new_execution(
      GasStationRequestErrorKind::BodyDeserialization { source: e },
      gas_station_url.clone(),
    )
  })?;

  let Some(effects) = effects else {
    return Err(GasStationRequestError::new_execution(
      GasStationRequestErrorKind::InvalidResponse {
        message: error,
        status_code,
      },
      gas_station_url.clone(),
    ));
  };

  Ok(effects)
}
