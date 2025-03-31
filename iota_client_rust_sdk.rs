// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use std::boxed::Box;
use std::marker::Send;
use std::option::Option;
use std::result::Result;

use secret_storage::Signer;

use crate::rebased::Error;
use identity_iota_interaction::apis::CoinReadApi;
use identity_iota_interaction::apis::EventApi;
use identity_iota_interaction::apis::QuorumDriverApi;
use identity_iota_interaction::apis::ReadApi;
use identity_iota_interaction::error::IotaRpcResult;
use identity_iota_interaction::rpc_types::Coin;
use identity_iota_interaction::rpc_types::CoinPage;
use identity_iota_interaction::rpc_types::EventFilter;
use identity_iota_interaction::rpc_types::EventPage;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaObjectResponse;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::rpc_types::IotaPastObjectResponse;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsV1;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::rpc_types::ObjectChange;
use identity_iota_interaction::rpc_types::ObjectsPage;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::types::digests::TransactionDigest;
use identity_iota_interaction::types::dynamic_field::DynamicFieldName;
use identity_iota_interaction::types::event::EventID;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::transaction::Transaction;
use identity_iota_interaction::types::transaction::TransactionData;
use identity_iota_interaction::types::transaction::TransactionDataAPI as _;
use identity_iota_interaction::CoinReadTrait;
use identity_iota_interaction::EventTrait;
use identity_iota_interaction::IotaClient;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::OptionalSync;
use identity_iota_interaction::QuorumDriverTrait;
use identity_iota_interaction::ReadTrait;

/// The minimum balance required to execute a transaction.
pub(crate) const MINIMUM_BALANCE: u64 = 1_000_000_000;

#[allow(unreachable_pub, dead_code)]
pub trait IotaTransactionBlockResponseAdaptedT:
  IotaTransactionBlockResponseT<Error = Error, NativeResponse = IotaTransactionBlockResponse>
{
}
impl<T> IotaTransactionBlockResponseAdaptedT for T where
  T: IotaTransactionBlockResponseT<Error = Error, NativeResponse = IotaTransactionBlockResponse>
{
}
#[allow(unreachable_pub, dead_code)]
pub type IotaTransactionBlockResponseAdaptedTraitObj =
  Box<dyn IotaTransactionBlockResponseT<Error = Error, NativeResponse = IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait QuorumDriverApiAdaptedT:
  QuorumDriverTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse>
{
}
impl<T> QuorumDriverApiAdaptedT for T where
  T: QuorumDriverTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse>
{
}
#[allow(unreachable_pub, dead_code)]
pub type QuorumDriverApiAdaptedTraitObj =
  Box<dyn QuorumDriverTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait ReadApiAdaptedT: ReadTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> {}
impl<T> ReadApiAdaptedT for T where T: ReadTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type ReadApiAdaptedTraitObj = Box<dyn ReadTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait CoinReadApiAdaptedT: CoinReadTrait<Error = Error> {}
impl<T> CoinReadApiAdaptedT for T where T: CoinReadTrait<Error = Error> {}
#[allow(unreachable_pub, dead_code)]
pub type CoinReadApiAdaptedTraitObj = Box<dyn CoinReadTrait<Error = Error>>;

#[allow(unreachable_pub, dead_code)]
pub trait EventApiAdaptedT: EventTrait<Error = Error> {}
impl<T> EventApiAdaptedT for T where T: EventTrait<Error = Error> {}
#[allow(unreachable_pub, dead_code)]
pub type EventApiAdaptedTraitObj = Box<dyn EventTrait<Error = Error>>;

#[allow(unreachable_pub, dead_code)]
pub trait IotaClientAdaptedT: IotaClientTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> {}
impl<T> IotaClientAdaptedT for T where T: IotaClientTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type IotaClientAdaptedTraitObj =
  Box<dyn IotaClientTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse>>;

pub(crate) struct IotaTransactionBlockResponseProvider {
  response: IotaTransactionBlockResponse,
}

impl IotaTransactionBlockResponseProvider {
  pub(crate) fn new(response: IotaTransactionBlockResponse) -> Self {
    IotaTransactionBlockResponseProvider { response }
  }
}

impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  fn effects(&self) -> Option<&IotaTransactionBlockEffects> {
    self.response.effects.as_ref()
  }

  fn to_string(&self) -> String {
    format!("{:?}", self.response)
  }

  fn as_native_response(&self) -> &Self::NativeResponse {
    &self.response
  }

  fn as_mut_native_response(&mut self) -> &mut Self::NativeResponse {
    &mut self.response
  }

  fn clone_native_response(&self) -> Self::NativeResponse {
    self.response.clone()
  }

  fn digest(&self) -> Result<TransactionDigest, Self::Error> {
    Ok(self.response.digest)
  }
}

pub(crate) struct QuorumDriverAdapter<'a> {
  api: &'a QuorumDriverApi,
}

#[async_trait::async_trait()]
impl QuorumDriverTrait for QuorumDriverAdapter<'_> {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  async fn execute_transaction_block(
    &self,
    tx_data: TransactionData,
    signatures: Vec<Signature>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let tx = Transaction::from_data(tx_data, signatures);
    let response = self
      .api
      .execute_transaction_block(tx, options.unwrap_or_default(), request_type)
      .await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }
}

pub(crate) struct ReadAdapter<'a> {
  api: &'a ReadApi,
}

#[async_trait::async_trait()]
impl ReadTrait for ReadAdapter<'_> {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
    self
      .api
      .get_chain_identifier()
      .await
      .map_err(|e| Error::Network("SDK get_chain_identifier() call failed".to_string(), e))
  }

  async fn get_dynamic_field_object(
    &self,
    parent_object_id: ObjectID,
    name: DynamicFieldName,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.api.get_dynamic_field_object(parent_object_id, name).await
  }

  async fn get_object_with_options(
    &self,
    object_id: ObjectID,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.api.get_object_with_options(object_id, options).await
  }

  async fn get_owned_objects(
    &self,
    address: IotaAddress,
    query: Option<IotaObjectResponseQuery>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<ObjectsPage> {
    self.api.get_owned_objects(address, query, cursor, limit).await
  }

  async fn get_reference_gas_price(&self) -> IotaRpcResult<u64> {
    self.api.get_reference_gas_price().await
  }

  async fn get_transaction_with_options(
    &self,
    digest: TransactionDigest,
    options: IotaTransactionBlockResponseOptions,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let response = self.api.get_transaction_with_options(digest, options).await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }

  async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    self.api.try_get_parsed_past_object(object_id, version, options).await
  }
}

pub(crate) struct CoinReadAdapter<'a> {
  api: &'a CoinReadApi,
}

#[async_trait::async_trait()]
impl CoinReadTrait for CoinReadAdapter<'_> {
  type Error = Error;

  async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage> {
    self.api.get_coins(owner, coin_type, cursor, limit).await
  }
}

pub(crate) struct EventAdapter<'a> {
  api: &'a EventApi,
}

#[async_trait::async_trait()]
impl EventTrait for EventAdapter<'_> {
  type Error = Error;

  async fn query_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> IotaRpcResult<EventPage> {
    self.api.query_events(query, cursor, limit, descending_order).await
  }
}

#[derive(Clone)]
pub struct IotaClientRustSdk {
  iota_client: IotaClient,
}

#[async_trait]
impl IotaClientTrait for IotaClientRustSdk {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  fn quorum_driver_api(
    &self,
  ) -> Box<dyn QuorumDriverTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> + Send + '_> {
    Box::new(QuorumDriverAdapter {
      api: self.iota_client.quorum_driver_api(),
    })
  }

  fn read_api(&self) -> Box<dyn ReadTrait<Error = Error, NativeResponse = IotaTransactionBlockResponse> + Send + '_> {
    Box::new(ReadAdapter {
      api: self.iota_client.read_api(),
    })
  }

  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + Send + '_> {
    Box::new(CoinReadAdapter {
      api: self.iota_client.coin_read_api(),
    })
  }

  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + Send + '_> {
    Box::new(EventAdapter {
      api: self.iota_client.event_api(),
    })
  }

  async fn execute_transaction<S>(
    &self,
    tx_data: TransactionData,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponseAdaptedTraitObj, Self::Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let response = self.sdk_execute_transaction(tx_data, signer).await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }

  async fn default_gas_budget(&self, sender_address: IotaAddress, tx: &ProgrammableTransaction) -> Result<u64, Error> {
    self.sdk_default_gas_budget(sender_address, tx).await
  }

  async fn get_previous_version(&self, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Error> {
    // try to get digest of previous tx
    // if we requested the prev tx and it isn't returned, this should be the oldest state
    let prev_tx_digest = if let Some(value) = iod.previous_transaction {
      value
    } else {
      return Ok(None);
    };

    // resolve previous tx
    let prev_tx_response = self
      .iota_client
      .read_api()
      .get_transaction_with_options(
        prev_tx_digest,
        IotaTransactionBlockResponseOptions::new().with_object_changes(),
      )
      .await
      .map_err(|err| {
        Error::InvalidIdentityHistory(format!("could not get previous transaction {prev_tx_digest}; {err}"))
      })?;

    // check for updated/created changes
    let (created, other_changes): (Vec<ObjectChange>, _) = prev_tx_response
      .clone()
      .object_changes
      .ok_or_else(|| {
        Error::InvalidIdentityHistory(format!(
          "could not find object changes for object {} in transaction {prev_tx_digest}",
          iod.object_id
        ))
      })?
      .into_iter()
      .filter(|elem| iod.object_id.eq(&elem.object_id()))
      .partition(|elem| matches!(elem, ObjectChange::Created { .. }));

    // previous tx contain create tx, so there is no previous version
    if created.len() == 1 {
      return Ok(None);
    }

    let mut previous_versions: Vec<SequenceNumber> = other_changes
      .iter()
      .filter_map(|elem| match elem {
        ObjectChange::Mutated { previous_version, .. } => Some(*previous_version),
        _ => None,
      })
      .collect();

    previous_versions.sort();

    let earliest_previous = if let Some(value) = previous_versions.first() {
      value
    } else {
      return Ok(None); // no mutations in prev tx, so no more versions can be found
    };

    let past_obj_response = self.get_past_object(iod.object_id, *earliest_previous).await?;
    match past_obj_response {
      IotaPastObjectResponse::VersionFound(value) => Ok(Some(value)),
      _ => Err(Error::InvalidIdentityHistory(format!(
        "could not find previous version, past object response: {past_obj_response:?}"
      ))),
    }
  }

  async fn get_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
  ) -> Result<IotaPastObjectResponse, Error> {
    self
      .iota_client
      .read_api()
      .try_get_parsed_past_object(object_id, version, IotaObjectDataOptions::full_content())
      .await
      .map_err(|err| {
        Error::InvalidIdentityHistory(format!("could not look up object {object_id} version {version}; {err}"))
      })
  }
}

impl IotaClientRustSdk {
  pub fn new(iota_client: IotaClient) -> Result<Self, Error> {
    Ok(Self { iota_client })
  }

  async fn sdk_execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    tx: TransactionData,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponse, Error> {
    let public_key = signer
      .public_key()
      .await
      .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
    let sender_address = IotaAddress::from(&public_key);

    if sender_address != tx.sender() {
      return Err(Error::TransactionSigningFailed(format!("transaction data needs to be signed by address {}, but client can only provide signature for address {sender_address}", tx.sender())));
    }

    let signature = signer
      .sign(&tx)
      .await
      .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;

    // execute tx
    let response = self
      .iota_client
      .quorum_driver_api()
      .execute_transaction_block(
        Transaction::from_data(tx, vec![signature]),
        IotaTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await
      .map_err(Error::TransactionExecutionFailed)?;

    if let Some(IotaTransactionBlockEffects::V1(IotaTransactionBlockEffectsV1 {
      status: IotaExecutionStatus::Failure { error },
      ..
    })) = &response.effects
    {
      Err(Error::TransactionUnexpectedResponse(error.to_string()))
    } else {
      Ok(response)
    }
  }

  async fn sdk_default_gas_budget(
    &self,
    sender_address: IotaAddress,
    tx: &ProgrammableTransaction,
  ) -> Result<u64, Error> {
    let gas_price = self
      .iota_client
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?;
    let gas_coin = self.get_coin_for_transaction(sender_address).await?;
    let tx_data = TransactionData::new_programmable(
      sender_address,
      vec![gas_coin.object_ref()],
      tx.clone(),
      50_000_000,
      gas_price,
    );
    let dry_run_gas_result = self
      .iota_client
      .read_api()
      .dry_run_transaction_block(tx_data)
      .await?
      .effects;
    if dry_run_gas_result.status().is_err() {
      let IotaExecutionStatus::Failure { error } = dry_run_gas_result.into_status() else {
        unreachable!();
      };
      return Err(Error::TransactionUnexpectedResponse(error));
    }
    let gas_summary = dry_run_gas_result.gas_cost_summary();
    let overhead = gas_price * 1000;
    let net_used = gas_summary.net_gas_usage();
    let computation = gas_summary.computation_cost;

    let budget = overhead + (net_used.max(0) as u64).max(computation);
    Ok(budget)
  }

  async fn get_coin_for_transaction(&self, sender_address: IotaAddress) -> Result<Coin, Error> {
    const LIMIT: usize = 10;
    let mut cursor = None;

    loop {
      let coins = self
        .iota_client
        .coin_read_api()
        .get_coins(sender_address, None, cursor, Some(LIMIT))
        .await?;

      let Some(coin) = coins.data.into_iter().max_by_key(|coin| coin.balance) else {
        return Err(Error::GasIssue(format!(
          "no coin found with minimum required balance of {} for address {}",
          MINIMUM_BALANCE, sender_address
        )));
      };

      if coin.balance >= MINIMUM_BALANCE {
        return Ok(coin);
      }

      if !coins.has_next_page {
        break;
      }

      cursor = coins.next_cursor;
    }

    Err(Error::GasIssue(format!(
      "no coin found with minimum required balance of {} for address {}",
      MINIMUM_BALANCE, sender_address
    )))
  }
}
