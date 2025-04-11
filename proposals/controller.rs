// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::migration::ControllerToken;
use crate::rebased::transaction_builder::Transaction;
use crate::rebased::transaction_builder::TransactionBuilder;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI as _;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::IdentityMoveCalls;
use tokio::sync::Mutex;

use crate::rebased::migration::Proposal;
use crate::rebased::transaction::ProtoTransaction;
use crate::rebased::Error;
use async_trait::async_trait;
use identity_iota_interaction::rpc_types::IotaObjectRef;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::transaction::Argument;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::MoveType;
use serde::Deserialize;
use serde::Serialize;

use super::CreateProposal;
use super::OnChainIdentity;
use super::ProposalBuilder;
use super::ProposalT;
use super::UserDrivenTx;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
      use iota_interaction_ts::NativeTsTransactionBuilderBindingWrapper as Ptb;
      /// Instances of ControllerIntentFnT can be used as user-provided function to describe how
      /// a borrowed identity's controller capability will be used.
      pub trait ControllerIntentFnT: FnOnce(&mut Ptb, &Argument) {}
      impl<T> ControllerIntentFnT for T where T: FnOnce(&mut Ptb, &Argument) {}
      #[allow(unreachable_pub)]
      /// Boxed dynamic trait object of {@link ControllerIntentFnT}
      pub type ControllerIntentFn = Box<dyn ControllerIntentFnT + Send>;
    } else {
      use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
      /// Instances of ControllerIntentFnT can be used as user-provided function to describe how
      /// a borrowed identity's controller capability will be used.
      pub trait ControllerIntentFnT: FnOnce(&mut Ptb, &Argument) {}
      impl<T> ControllerIntentFnT for T where T: FnOnce(&mut Ptb, &Argument) {}
      #[allow(unreachable_pub)]
      /// Boxed dynamic trait object of {@link ControllerIntentFnT}
      pub type ControllerIntentFn = Box<dyn ControllerIntentFnT + Send>;
    }
}

/// Borrow an [`OnChainIdentity`]'s controller capability to exert control on
/// a sub-owned identity.
#[derive(Debug, Deserialize, Serialize)]
pub struct ControllerExecution<F = ControllerIntentFn> {
  controller_cap: ObjectID,
  identity: IotaAddress,
  #[serde(skip, default = "Mutex::default")]
  intent_fn: Mutex<Option<F>>,
}

/// A [`ControllerExecution`] action coupled with a user-provided function to describe how
/// the borrowed identity's controller capability will be used.
pub struct ControllerExecutionWithIntent<F>(ControllerExecution<F>)
where
  F: FnOnce(&mut Ptb, &Argument);

impl<F> ControllerExecutionWithIntent<F>
where
  F: ControllerIntentFnT,
{
  fn new(mut action: ControllerExecution<F>) -> Self {
    debug_assert!(action.intent_fn.get_mut().is_some());
    Self(action)
  }
}

impl<F> ControllerExecution<F> {
  /// Creates a new [`ControllerExecution`] action, allowing a controller of `identity` to
  /// borrow `identity`'s controller cap for a transaction.
  pub fn new(controller_cap: ObjectID, identity: &OnChainIdentity) -> Self {
    Self {
      controller_cap,
      identity: identity.id().into(),
      intent_fn: Mutex::default(),
    }
  }

  /// Specifies how the borrowed `ControllerCap` should be used in the transaction.
  /// This is only useful if the controller creating this proposal has enough voting
  /// power to carry out it out immediately.
  pub fn with_intent<F1>(self, intent_fn: F1) -> ControllerExecution<F1>
  where
    F1: FnOnce(&mut Ptb, &Argument),
  {
    let Self {
      controller_cap,
      identity,
      ..
    } = self;
    ControllerExecution {
      controller_cap,
      identity,
      intent_fn: Mutex::new(Some(intent_fn)),
    }
  }
}

impl<'i, 'c, F> ProposalBuilder<'i, 'c, ControllerExecution<F>> {
  /// Specifies how the borrowed `ControllerCap` should be used in the transaction.
  /// This is only useful if the controller creating this proposal has enough voting
  /// power to carry out it out immediately.
  pub fn with_intent<F1>(self, intent_fn: F1) -> ProposalBuilder<'i, 'c, ControllerExecution<F1>>
  where
    F1: FnOnce(&mut Ptb, &Argument),
  {
    let ProposalBuilder {
      identity,
      controller_token,
      expiration,
      action,
    } = self;
    ProposalBuilder {
      identity,
      controller_token,
      expiration,
      action: action.with_intent(intent_fn),
    }
  }
}

impl MoveType for ControllerExecution {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::controller_proposal::ControllerExecution")).expect("valid move type")
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl<F> ProposalT for Proposal<ControllerExecution<F>>
where
  F: ControllerIntentFnT + Send,
{
  type Action = ControllerExecution<F>;
  type Output = ();

  async fn create<'i>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    client: &IdentityClientReadOnly,
  ) -> Result<TransactionBuilder<CreateProposal<'i, Self::Action>>, Error> {
    if identity.id() != controller_token.controller_of() {
      return Err(Error::Identity(format!(
        "token {} doesn't grant access to identity {}",
        controller_token.id(),
        identity.id()
      )));
    }
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = client
      .get_object_ref_by_id(controller_token.id())
      .await?
      .ok_or_else(|| Error::Identity(format!("controller token {} doesn't exists", controller_token.id())))?
      .reference
      .to_object_ref();
    let maybe_intent_fn = action.intent_fn.into_inner();
    let chained_execution = maybe_intent_fn.is_some()
      && identity
        .controller_voting_power(controller_cap_ref.0)
        .expect("is an identity's controller")
        >= identity.threshold();

    let ptb = if chained_execution {
      let borrowing_controller_cap_ref = client
        .get_object_ref_by_id(action.controller_cap)
        .await?
        .map(|OwnedObjectRef { reference, .. }| {
          let IotaObjectRef {
            object_id,
            version,
            digest,
          } = reference;
          (object_id, version, digest)
        })
        .ok_or_else(|| Error::ObjectLookup(format!("object {} doesn't exist", action.controller_cap)))?;

      IdentityMoveCallsAdapter::create_and_execute_controller_execution(
        identity_ref,
        controller_cap_ref,
        expiration,
        borrowing_controller_cap_ref,
        maybe_intent_fn.unwrap(),
        client.package_id(),
      )
    } else {
      IdentityMoveCallsAdapter::propose_controller_execution(
        identity_ref,
        controller_cap_ref,
        action.controller_cap,
        expiration,
        client.package_id(),
      )
    }
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(TransactionBuilder::new(CreateProposal {
      identity,
      ptb: bcs::from_bytes(&ptb)?,
      chained_execution,
      _action: PhantomData,
    }))
  }

  async fn into_tx<'i>(
    self,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    _: &IdentityClientReadOnly,
  ) -> Result<UserDrivenTx<'i, Self::Action>, Error> {
    if identity.id() != controller_token.controller_of() {
      return Err(Error::Identity(format!(
        "token {} doesn't grant access to identity {}",
        controller_token.id(),
        identity.id()
      )));
    }

    let proposal_id = self.id();
    let controller_execution_action = self.into_action();

    Ok(UserDrivenTx::new(
      identity,
      controller_token.id(),
      controller_execution_action,
      proposal_id,
    ))
  }

  fn parse_tx_effects(effects: &IotaTransactionBlockEffects) -> Result<Self::Output, Error> {
    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    Ok(())
  }
}

impl<'i, F> UserDrivenTx<'i, ControllerExecution<F>> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F1>(self, intent_fn: F1) -> UserDrivenTx<'i, ControllerExecutionWithIntent<F1>>
  where
    F1: ControllerIntentFnT,
  {
    let UserDrivenTx {
      identity,
      action,
      controller_token,
      proposal_id,
      ..
    } = self;

    UserDrivenTx::new(
      identity,
      controller_token,
      ControllerExecutionWithIntent::new(action.with_intent(intent_fn)),
      proposal_id,
    )
  }
}

impl<'i, F> ProtoTransaction for UserDrivenTx<'i, ControllerExecution<F>> {
  type Input = ControllerIntentFn;
  type Tx = TransactionBuilder<UserDrivenTx<'i, ControllerExecutionWithIntent<ControllerIntentFn>>>;

  fn with(self, input: Self::Input) -> Self::Tx {
    TransactionBuilder::new(self.with_intent(input))
  }
}

impl<F> UserDrivenTx<'_, ControllerExecutionWithIntent<F>>
where
  F: ControllerIntentFnT + Send,
{
  async fn make_ptb(&self, client: &IdentityClientReadOnly) -> Result<ProgrammableTransaction, Error> {
    let Self {
      identity,
      action,
      controller_token,
      proposal_id,
      ..
    } = self;
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = client
      .get_object_ref_by_id(*controller_token)
      .await?
      .ok_or_else(|| Error::Identity(format!("controller token {} doesn't exists", controller_token)))?
      .reference
      .to_object_ref();

    let borrowing_cap_id = action.0.controller_cap;
    let borrowing_controller_cap_ref = client
      .get_object_ref_by_id(borrowing_cap_id)
      .await?
      .map(|object_ref| object_ref.reference.to_object_ref())
      .ok_or_else(|| Error::ObjectLookup(format!("object {borrowing_cap_id} doesn't exist")))?;

    let tx = IdentityMoveCallsAdapter::execute_controller_execution(
      identity_ref,
      controller_cap_ref,
      *proposal_id,
      borrowing_controller_cap_ref,
      action
        .0
        .intent_fn
        .lock()
        .await
        .take()
        .expect("BorrowActionWithIntent makes sure intent_fn is present"),
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(bcs::from_bytes(&tx)?)
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl<F> Transaction for UserDrivenTx<'_, ControllerExecutionWithIntent<F>>
where
  F: ControllerIntentFnT + Send,
{
  type Output = ();
  async fn build_programmable_transaction(
    &self,
    client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    self.cached_ptb.get_or_try_init(|| self.make_ptb(client)).await.cloned()
  }
  async fn apply(self, effects: &IotaTransactionBlockEffects, _client: &IdentityClientReadOnly) -> Result<(), Error> {
    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    Ok(())
  }
}
