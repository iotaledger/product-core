// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use anyhow::Context as _;
use async_trait::async_trait;
use cfg_if::cfg_if;
use iota_interaction::rpc_types::{
  IotaTransactionBlockEffects, IotaTransactionBlockEffectsAPI as _, IotaTransactionBlockEvents,
  IotaTransactionBlockResponseOptions,
};
use iota_interaction::shared_crypto::intent::{Intent, IntentMessage};
use iota_interaction::types::base_types::{IotaAddress, ObjectRef};
use iota_interaction::types::crypto::{IotaSignature as _, PublicKey, Signature};
use iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use iota_interaction::types::transaction::{
  GasData, ProgrammableTransaction, TransactionData, TransactionDataAPI as _, TransactionKind,
};
use iota_interaction::{IotaClientTrait, IotaKeySignature, OptionalSend, OptionalSync};
use itertools::Itertools;
use secret_storage::Signer;

#[cfg(not(target_arch = "wasm32"))]
use super::TransactionOutput;
#[cfg(target_arch = "wasm32")]
use super::TransactionOutputInternal as TransactionOutput;
use crate::core_client::{CoreClient, CoreClientReadOnly};
use crate::Error;

/// An operation that combines a transaction with its off-chain effects.
#[cfg_attr(feature = "send-sync", async_trait)]
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
pub trait Transaction: Sized {
  /// Error type for this transaction.
  type Error: std::error::Error + Sync + Send + 'static;
  /// Output type for this transaction.
  type Output;

  /// Encode this operation into a [ProgrammableTransaction].
  async fn build_programmable_transaction<C>(&self, client: &C) -> Result<ProgrammableTransaction, Self::Error>
  where
    C: CoreClientReadOnly + OptionalSync;

  /// Parses a transaction result in order to compute its effects.
  /// ## Notes
  /// [Transaction::apply] implementations should make sure to properly consume
  /// the parts of `effects` that are needed for the transaction - e.g., removing
  /// the ID of the object the transaction created from the `effects`'s list of
  /// created objects.
  /// This is particularly important to enable the batching of transactions.
  async fn apply<C>(self, effects: &mut IotaTransactionBlockEffects, client: &C) -> Result<Self::Output, Self::Error>
  where
    C: CoreClientReadOnly + OptionalSync;

  /// Parses a transaction result in order to compute its effects and optionally use events.
  ///
  /// This method is a convenience wrapper around [`Transaction::apply`] that passes the
  /// effects and events to the transaction logic. By default, this implementation ignores
  /// the `events` parameter and simply calls [`apply`].
  ///
  /// ## Handling Events
  ///
  /// If you need to handle events in your transaction logic, override this
  /// method and process the effects and events in this function. Also make
  /// sure to return an appropriate error in your [`apply`] function implementation, since users  could still call
  /// `apply` directly in their own code.
  ///
  /// ## Important Notes
  ///
  /// Although users are not expected to call the `apply` function directly, it
  /// is possible.
  /// Therefore, always ensure that `apply` returns a meaningful error if
  /// called in a context
  /// where event handling is required, rather than panicking or failing
  /// silently. This improves debuggability and prevents silent failures.
  async fn apply_with_events<C>(
    self,
    effects: &mut IotaTransactionBlockEffects,
    events: &mut IotaTransactionBlockEvents,
    client: &C,
  ) -> Result<Self::Output, Self::Error>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    let _ = events;

    self.apply(effects, client).await
  }
}

#[derive(Debug, Default, Clone)]
struct PartialGasData {
  payment: Vec<ObjectRef>,
  owner: Option<IotaAddress>,
  price: Option<u64>,
  budget: Option<u64>,
}

impl From<GasData> for PartialGasData {
  fn from(value: GasData) -> Self {
    Self {
      payment: value.payment,
      owner: Some(value.owner),
      price: Some(value.price),
      budget: Some(value.budget),
    }
  }
}

impl PartialGasData {
  fn into_gas_data_with_defaults(self) -> GasData {
    GasData {
      payment: self.payment,
      owner: self.owner.unwrap_or_default(),
      price: self.price.unwrap_or_default(),
      budget: self.budget.unwrap_or_default(),
    }
  }
}

impl TryFrom<PartialGasData> for GasData {
  type Error = Error;
  fn try_from(value: PartialGasData) -> Result<Self, Self::Error> {
    let owner = value
      .owner
      .ok_or_else(|| Error::GasIssue("missing gas owner".to_owned()))?;
    let price = value
      .price
      .ok_or_else(|| Error::GasIssue("missing gas price".to_owned()))?;
    let budget = value
      .budget
      .ok_or_else(|| Error::GasIssue("missing gas budget".to_owned()))?;

    Ok(GasData {
      payment: value.payment,
      owner,
      price,
      budget,
    })
  }
}

/// A reference to [TransactionData] that only allows to mutate its [GasData].
#[derive(Debug)]
pub struct MutGasDataRef<'tx>(&'tx mut TransactionData);
impl Deref for MutGasDataRef<'_> {
  type Target = TransactionData;
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl MutGasDataRef<'_> {
  /// Returns a mutable reference to [GasData].
  pub fn gas_data_mut(&mut self) -> &mut GasData {
    self.0.gas_data_mut()
  }
}

/// Builds an executable transaction on a step by step manner.
#[derive(Debug)]
pub struct TransactionBuilder<Tx> {
  programmable_tx: Option<ProgrammableTransaction>,
  sender: Option<IotaAddress>,
  gas: PartialGasData,
  signatures: Vec<Signature>,
  tx: Tx,
}

impl<Tx> AsRef<Tx> for TransactionBuilder<Tx> {
  fn as_ref(&self) -> &Tx {
    &self.tx
  }
}

impl<Tx> TransactionBuilder<Tx>
where
  Tx: Transaction + OptionalSend,
{
  /// Starts the creation of an executable transaction by supplying
  /// a type implementing [Transaction].
  pub fn new(effect: Tx) -> Self {
    Self {
      tx: effect,
      gas: PartialGasData::default(),
      signatures: vec![],
      sender: None,
      programmable_tx: None,
    }
  }

  async fn transaction_data<C>(&mut self, client: &C) -> anyhow::Result<TransactionData>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    // Make sure the partial gas information is actually complete to create a whole GasData.
    let gas_data: GasData = std::mem::take(&mut self.gas).try_into()?;
    self.gas = gas_data.into();

    // Forward call to "with_partial_gas" knowing no defaults will be used.
    self.transaction_data_with_partial_gas(client).await
  }

  /// Same as [Self::transaction_data] but will not fail with incomplete gas information.
  /// Missing gas data is filled with default values through [PartialGasData::into_gas_data_with_defaults].
  async fn transaction_data_with_partial_gas<C>(&mut self, client: &C) -> anyhow::Result<TransactionData>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    let sender = self.sender.context("missing sender")?;
    let gas_data = self.gas.clone().into_gas_data_with_defaults();
    let pt = self.get_or_init_programmable_tx(client).await?.clone();

    Ok(TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(pt),
      sender,
      gas_data,
    ))
  }

  /// Adds `signer`'s signature to this transaction's signatures' list.
  /// # Notes
  /// This method asserts that `signer`'s address matches the address of
  /// either this transaction's sender or the gas owner - failing otherwise.
  pub async fn with_signature<C, S>(mut self, client: &C) -> Result<Self, Error>
  where
    C: CoreClient<S> + OptionalSync,
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let pk = client
      .signer()
      .public_key()
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let signer_address = IotaAddress::from(&pk);

    let matches_sender = self.sender.map_or(true, |sender| sender == signer_address);
    let matches_gas_owner = self.gas.owner.map_or(true, |owner| owner == signer_address);

    if !(matches_sender || matches_gas_owner) {
      return Err(Error::TransactionBuildingFailed(format!(
        "signer's address {signer_address} doesn't match the address of either the transaction sender or the gas owner"
      )));
    }

    let tx_data = self
      .transaction_data(client)
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    let sig = client
      .signer()
      .sign(&tx_data)
      .await
      .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
    self.signatures.push(sig);

    Ok(self)
  }

  /// Attempts to sponsor this transaction by having another party supply [GasData] and gas owner signature.
  /// ## Notes
  /// The [TransactionData] passed to `sponsor_tx` can be constructed from partial gas data; the sponsor is
  /// tasked with setting the gas information appropriately before signing.
  pub async fn with_sponsor<C, F>(mut self, client: &C, sponsor_tx: F) -> Result<Self, Error>
  where
    C: CoreClientReadOnly + OptionalSync,
    F: AsyncFnOnce(MutGasDataRef<'_>) -> anyhow::Result<Signature>,
  {
    let mut tx_data = self
      .transaction_data_with_partial_gas(client)
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    let signature = sponsor_tx(MutGasDataRef(&mut tx_data))
      .await
      .map_err(|e| Error::GasIssue(format!("failed to sponsor transaction: {e}")))?;

    let gas_owner = tx_data.gas_owner();
    let mut intent_msg = IntentMessage::new(Intent::iota_transaction(), tx_data);
    signature
      .verify_secure(&intent_msg, gas_owner, signature.scheme())
      .map_err(|e| Error::TransactionBuildingFailed(format!("invalid sponsor signature: {e}")))?;
    let gas_data = std::mem::replace(
      intent_msg.value.gas_data_mut(),
      GasData {
        payment: vec![],
        owner: IotaAddress::ZERO,
        price: 0,
        budget: 0,
      },
    );

    self.signatures.push(signature);
    self.gas = gas_data.into();

    Ok(self)
  }

  async fn get_or_init_programmable_tx<C>(&mut self, client: &C) -> Result<&ProgrammableTransaction, Error>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    if self.programmable_tx.is_none() {
      self.programmable_tx = Some(
        self
          .tx
          .build_programmable_transaction(client)
          .await
          .map_err(|e| Error::Transaction(Box::new(e)))?,
      );
    }

    Ok(self.programmable_tx.as_ref().unwrap())
  }

  /// Similar to [Self::build] but missing values are replaced by defaults.
  pub async fn build_with_defaults<C>(mut self, client: &C) -> Result<(TransactionData, Vec<Signature>, Tx), Error>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    if self.sender.is_none() {
      self.sender = Some(IotaAddress::default());
    }
    let tx_data = self
      .transaction_data_with_partial_gas(client)
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok((tx_data, self.signatures, self.tx))
  }

  /// Attempts to build this transaction using `client` in a best effort manner:
  /// - when no sender had been supplied, client's address is used;
  /// - when gas information is incomplete, the client will attempt to fill it, making use of whatever funds its address
  ///   has, if possible;
  /// - when signatures are missing, the client will provide its own if possible;
  ///
  /// ## Notes
  /// This method *DOES NOT* remove nor checks for invalid signatures.
  /// Transaction with invalid signatures will fail after attempting to execute them.
  pub async fn build<C, S>(mut self, client: &C) -> Result<(TransactionData, Vec<Signature>, Tx), Error>
  where
    C: CoreClient<S> + OptionalSync,
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    self.get_or_init_programmable_tx(client).await?;
    let programmable_tx = self.programmable_tx.expect("just computed it");
    let client_address = client.sender_address();
    let sender = self.sender.unwrap_or(client_address);
    let gas_data = complete_gas_data_for_tx(&programmable_tx, self.gas, client)
      .await
      .map_err(|e| Error::GasIssue(e.to_string()))?;

    let tx_data = TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(programmable_tx),
      sender,
      gas_data,
    );

    let mut signatures = self.signatures;
    let needs_client_signature = client_address == sender
      || client_address == tx_data.gas_data().owner
        && !signatures.iter().map(address_from_signature).contains(&client_address);
    if needs_client_signature {
      let signature = client
        .signer()
        .sign(&tx_data)
        .await
        .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
      signatures.push(signature);
    }

    Ok((tx_data, signatures, self.tx))
  }

  /// Attempts to build and execute this transaction using `client` in a best effort manner:
  /// - when no sender had been supplied, client's address is used;
  /// - when gas information is incomplete, the client will attempt to fill it, making use of whatever funds its address
  ///   has, if possible;
  /// - when signatures are missing, the client will provide its own if possible;
  ///
  /// After the transaction has been successfully executed, the transaction's effect will be computed.
  /// ## Notes
  /// This method *DOES NOT* remove nor checks for invalid signatures.
  /// Transactions with invalid signatures will fail after attempting to execute them.
  pub async fn build_and_execute<C, S>(self, client: &C) -> Result<TransactionOutput<Tx::Output>, Error>
  where
    C: CoreClient<S> + OptionalSync,
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    // Build the transaction into its parts.
    let (tx_data, signatures, tx) = self.build(client).await?;

    // Execute and wait for the transaction to be confirmed.
    let dyn_tx_block = client
      .client_adapter()
      .quorum_driver_api()
      .execute_transaction_block(
        tx_data,
        signatures,
        Some(IotaTransactionBlockResponseOptions::full_content()),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    // Get the transaction's effects, making sure they are successful.
    let mut tx_effects = dyn_tx_block
      .effects()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects in response".to_owned()))?
      .clone();
    let tx_status = tx_effects.status();
    if tx_status.is_err() {
      return Err(Error::TransactionUnexpectedResponse(format!(
        "errors in transaction's effects: {}",
        tx_status
      )));
    }

    let application_result = tx
      .apply_with_events(
        &mut tx_effects,
        &mut dyn_tx_block.events().cloned().unwrap_or_default(),
        client,
      )
      .await;
    let response = {
      cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
          dyn_tx_block
        } else {
          dyn_tx_block.clone_native_response()
        }
      }
    };
    // Apply the off-chain logic of the transaction by parsing the transaction's effects.
    // If the application goes awry, salvage the response by returning it alongside the error.
    let output = match application_result {
      Ok(output) => output,
      Err(e) => {
        #[cfg(not(target_arch = "wasm32"))]
        let response = Box::new(response);
        #[cfg(target_arch = "wasm32")]
        // For WASM the response is passed in the error as its JSON-encoded string representation.
        let response = response.as_native_response().to_string();
        return Err(Error::TransactionOffChainApplicationFailure {
          source: Box::new(Error::Transaction(Box::new(e))),
          response,
        });
      }
    };

    Ok(TransactionOutput { output, response })
  }
}

impl<Tx> TransactionBuilder<Tx> {
  /// Returns the partial [Transaction] wrapped by this builder, consuming it.
  pub fn into_inner(self) -> Tx {
    self.tx
  }

  /// Sets the address that will execute the transaction.
  pub fn with_sender(mut self, sender: IotaAddress) -> Self {
    self.sender = Some(sender);
    self
  }

  /// Sets the gas budget for this transaction.
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.gas.budget = Some(budget);
    self
  }

  /// Sets the coins to use to cover the gas cost.
  pub fn with_gas_payment(mut self, coins: Vec<ObjectRef>) -> Self {
    self.gas.payment = coins;
    self
  }

  /// Sets the gas owner.
  pub fn with_gas_owner(mut self, address: IotaAddress) -> Self {
    self.gas.owner = Some(address);
    self
  }

  /// Sets the gas price.
  pub fn with_gas_price(mut self, price: u64) -> Self {
    self.gas.price = Some(price);
    self
  }

  /// Sets the gas information that must be used to execute this transaction.
  pub fn with_gas_data(mut self, gas_data: GasData) -> Self {
    self.gas = gas_data.into();
    self
  }

  /// Attempts to construct a [TransactionBuilder] from a whole transaction.
  pub fn try_from_signed_transaction(
    tx_data: TransactionData,
    signatures: Vec<Signature>,
    effect: Tx,
  ) -> Result<Self, Error> {
    #[allow(irrefutable_let_patterns)]
    let TransactionKind::ProgrammableTransaction(pt) = tx_data.kind().clone() else {
      return Err(Error::TransactionBuildingFailed(
        "only programmable transactions are supported".to_string(),
      ));
    };
    let sender = tx_data.sender();
    let gas = tx_data.gas_data().clone().into();

    Ok(Self {
      programmable_tx: Some(pt),
      sender: Some(sender),
      gas,
      signatures,
      tx: effect,
    })
  }
}

/// Returns a best effort [GasData] for the given transaction, partial gas information, and client.
/// ## Notes
/// If a field is missing from gas data:
/// - client's address is set as the gas owner;
/// - current gas price is fetched from a node;
/// - budget is calculated by dry running the transaction;
/// - payment is set to whatever IOTA coins the gas owner has, that satisfy the tx's budget;
async fn complete_gas_data_for_tx<C, S>(
  pt: &ProgrammableTransaction,
  partial_gas_data: PartialGasData,
  client: &C,
) -> anyhow::Result<GasData>
where
  C: CoreClient<S> + OptionalSync,
  S: Signer<IotaKeySignature>,
{
  let owner = partial_gas_data.owner.unwrap_or(client.sender_address());
  let price = if let Some(price) = partial_gas_data.price {
    price
  } else {
    client.client_adapter().read_api().get_reference_gas_price().await?
  };
  let budget = if let Some(budget) = partial_gas_data.budget {
    budget
  } else {
    client.client_adapter().default_gas_budget(owner, pt).await?
  };
  let payment = if !partial_gas_data.payment.is_empty() {
    partial_gas_data.payment
  } else {
    client.get_iota_coins_with_at_least_balance(owner, budget).await?
  };

  Ok(GasData {
    owner,
    payment,
    price,
    budget,
  })
}

/// Extract the signer's address from an IOTA [Signature].
fn address_from_signature(signature: &Signature) -> IotaAddress {
  let scheme = signature.scheme();
  let pk_bytes = signature.public_key_bytes();
  let pk = PublicKey::try_from_bytes(scheme, pk_bytes).expect("valid signature hence valid key");

  IotaAddress::from(&pk)
}

#[cfg(feature = "gas-station")]
pub mod gas_station {
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
  use iota_interaction::{IotaKeySignature, OptionalSend, OptionalSync};
  use secret_storage::Signer;
  use serde::{Deserialize, Serialize};

  use super::{Transaction, TransactionBuilder};
  use crate::core_client::CoreClient;
  use crate::http_client::{HttpClient, Method, Request, Url, UrlParsingError};
  use crate::Error;

  const DEFAULT_GAS_RESERVATION_DURATION: u64 = 60; // 1 minute.
  const DEFAULT_GAS_BUDGET_RESERVATION: u64 = 1_000_000_000; // 1 IOTA.

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
    fn new(kind: ErrorKind) -> Self {
      Self { kind }
    }

    #[inline(always)]
    fn from_reservation_error(e: GasStationRequestError) -> Self {
      Self {
        kind: ErrorKind::GasReservation(e),
      }
    }

    #[inline(always)]
    fn from_tx_execution_error(e: GasStationRequestError) -> Self {
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

  impl<Tx> TransactionBuilder<Tx>
  where
    Tx: Transaction + OptionalSend,
  {
    /// Execute this transaction using an IOTA Gas Station.
    #[cfg(not(feature = "default-http-client"))]
    pub async fn execute_with_gas_station<C, S, H>(
      self,
      client: &C,
      gas_station_url: &str,
      http_client: &H,
      options: Option<GasStationOptions>,
    ) -> Result<Tx::Output, GasStationError>
    where
      C: CoreClient<S> + OptionalSync,
      S: Signer<IotaKeySignature> + OptionalSync,
      H: HttpClient,
      H::Error: Into<Box<dyn error::Error + Send + Sync>>,
    {
      let gas_station_url = Url::parse(gas_station_url).map_err(|e| GasStationError::new(ErrorKind::Url(e)))?;
      execute_with_gas_station_impl(self, client, &gas_station_url, http_client, options.unwrap_or_default()).await
    }

    /// Execute this transaction using an IOTA Gas Station.
    #[cfg(feature = "default-http-client")]
    pub async fn execute_with_gas_station<C, S>(
      self,
      client: &C,
      gas_station_url: &str,
      options: Option<GasStationOptions>,
    ) -> Result<Tx::Output, GasStationError>
    where
      C: CoreClient<S> + OptionalSync,
      S: Signer<IotaKeySignature> + OptionalSync,
    {
      let gas_station_url = Url::parse(gas_station_url).map_err(|e| GasStationError::new(ErrorKind::Url(e)))?;
      let http_client = reqwest::Client::new();
      execute_with_gas_station_impl(
        self,
        client,
        &gas_station_url,
        &http_client,
        options.unwrap_or_default(),
      )
      .await
    }
  }
  /// Optional configuration to be passed to the gas-station when sponsoring a transaction.
  #[non_exhaustive]
  pub struct GasStationOptions {
    /// Duration of the gas allocation. Default value: `60` seconds.
    pub gas_reservation_duration: Duration,
    /// Bearer token to be included in all requests' "Authentication" header.
    pub bearer_auth: Option<String>,
  }

  impl Default for GasStationOptions {
    fn default() -> Self {
      Self {
        gas_reservation_duration: Duration::from_secs(DEFAULT_GAS_RESERVATION_DURATION),
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

  #[inline(always)]
  async fn execute_with_gas_station_impl<C, S, Tx, H>(
    mut tx_builder: TransactionBuilder<Tx>,
    client: &C,
    gas_station_url: &Url,
    http_client: &H,
    gas_station_options: GasStationOptions,
  ) -> Result<Tx::Output, GasStationError>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
    C: CoreClient<S> + OptionalSync,
    Tx: Transaction + OptionalSend,
    H: HttpClient<Error: Into<Box<dyn error::Error + Sync + Send>>>,
  {
    // Compute the arguments for gas reservation.
    let reserve_duration_secs = gas_station_options.gas_reservation_duration.as_secs();
    let gas_budget = tx_builder.gas.budget.unwrap_or(DEFAULT_GAS_BUDGET_RESERVATION);
    let auth_token = gas_station_options.bearer_auth.as_deref();

    // Get a gas reservation.
    let ReserveGasResult {
      sponsor_address,
      reservation_id,
      gas_coins,
    } = reserve_gas(
      gas_station_url,
      gas_budget,
      reserve_duration_secs,
      auth_token,
      http_client,
    )
    .await
    .map_err(GasStationError::from_reservation_error)?;
    // Map coins to known format.
    let gas_coins = gas_coins
      .into_iter()
      .map(
        |IotaObjectRef {
           object_id,
           version,
           digest,
         }| (object_id, version, digest),
      )
      .collect();

    // Set sponsor information in tx's gas data.
    // Note: gas' price can be set automatically.
    tx_builder.gas.owner = Some(sponsor_address);
    tx_builder.gas.payment = gas_coins;
    tx_builder.gas.budget = Some(gas_budget);

    // Consume the builder into its parts.
    let (tx_data, mut sigs, tx) = tx_builder
      .build(client)
      .await
      .map_err(|e| GasStationError::new(ErrorKind::TxDataBuilding(Box::new(e))))?;

    // Let gas-station execute this transaction.
    let mut effects = execute_sponsored_tx(
      gas_station_url,
      tx_data,
      sigs.pop().expect("signed by the sender"),
      reservation_id,
      auth_token,
      http_client,
    )
    .await
    .map_err(GasStationError::from_tx_execution_error)?;

    // Apply tx's side-effects.
    tx.apply(&mut effects, client)
      .await
      .map_err(|e| Error::Transaction(e.into()))
      .map_err(|e| GasStationError::new(ErrorKind::TxApplication(Box::new(e))))
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
  struct ReserveGasResult {
    sponsor_address: IotaAddress,
    reservation_id: u64,
    gas_coins: Vec<IotaObjectRef>,
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

  async fn reserve_gas<H>(
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
  struct ExecuteTxResponse {
    effects: Option<IotaTransactionBlockEffects>,
    error: Option<String>,
  }

  async fn execute_sponsored_tx<H>(
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
}
