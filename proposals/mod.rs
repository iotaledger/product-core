// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod borrow;
mod config_change;
mod controller;
mod send;
mod update_did_doc;
mod upgrade;

use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;

use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::OptionalSend;
use identity_iota_interaction::OptionalSync;
use tokio::sync::OnceCell;

use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::migration::get_identity;
use crate::rebased::transaction::ProtoTransaction;
use crate::rebased::transaction_builder::Transaction;
use crate::rebased::transaction_builder::TransactionBuilder;
use async_trait::async_trait;
pub use borrow::*;
pub use config_change::*;
pub use controller::*;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI as _;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::ObjectType;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::TypeTag;
pub use send::*;
use serde::de::DeserializeOwned;
pub use update_did_doc::*;
pub use upgrade::*;

use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use crate::rebased::Error;
use identity_iota_interaction::MoveType;

use super::migration::ControllerToken;

/// Interface that allows the creation and execution of an [`OnChainIdentity`]'s [`Proposal`]s.
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
pub trait ProposalT: Sized {
  /// The [`Proposal`] action's type.
  type Action;
  /// The output of the [`Proposal`]
  type Output;

  /// Creates a new [`Proposal`] with the provided action and expiration.
  async fn create<'i>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    client: &IdentityClientReadOnly,
  ) -> Result<TransactionBuilder<CreateProposal<'i, Self::Action>>, Error>;

  /// Converts the [`Proposal`] into a transaction that can be executed.
  async fn into_tx<'i>(
    self,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    client: &IdentityClientReadOnly,
  ) -> Result<impl ProtoTransaction, Error>;

  /// Parses the transaction's effects and returns the output of the [`Proposal`].
  fn parse_tx_effects(effects: &IotaTransactionBlockEffects) -> Result<Self::Output, Error>;
}

impl<A> Proposal<A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: MoveType + OptionalSend + OptionalSync,
{
  /// Creates a new [ApproveProposal] for the provided [`Proposal`].
  pub fn approve<'i>(
    &mut self,
    identity: &'i OnChainIdentity,
    controller_token: &ControllerToken,
  ) -> Result<TransactionBuilder<ApproveProposal<'_, 'i, A>>, Error> {
    ApproveProposal::new(self, identity, controller_token).map(TransactionBuilder::new)
  }
}

/// A builder for creating a [`Proposal`].
#[derive(Debug)]
pub struct ProposalBuilder<'i, 'c, A> {
  identity: &'i mut OnChainIdentity,
  controller_token: &'c ControllerToken,
  expiration: Option<u64>,
  action: A,
}

impl<A> Deref for ProposalBuilder<'_, '_, A> {
  type Target = A;
  fn deref(&self) -> &Self::Target {
    &self.action
  }
}

impl<A> DerefMut for ProposalBuilder<'_, '_, A> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.action
  }
}

impl<'i, 'c, A> ProposalBuilder<'i, 'c, A> {
  pub(crate) fn new(identity: &'i mut OnChainIdentity, controller_token: &'c ControllerToken, action: A) -> Self {
    Self {
      identity,
      controller_token,
      expiration: None,
      action,
    }
  }

  /// Sets the expiration epoch for the [`Proposal`].
  pub fn expiration_epoch(mut self, exp: u64) -> Self {
    self.expiration = Some(exp);
    self
  }

  /// Creates a [`Proposal`] with the provided arguments. If `forbid_chained_execution` is set to `true`,
  /// the [`Proposal`] won't be executed even if creator alone has enough voting power.
  pub async fn finish(self, client: &IdentityClientReadOnly) -> Result<TransactionBuilder<CreateProposal<'i, A>>, Error>
  where
    Proposal<A>: ProposalT<Action = A>,
  {
    let Self {
      action,
      expiration,
      controller_token,
      identity,
    } = self;

    Proposal::<A>::create(action, expiration, identity, controller_token, client).await
  }
}

#[derive(Debug)]
/// The result of creating a [`Proposal`]. When a [`Proposal`] is executed
/// in the same transaction as its creation, a [`ProposalResult::Executed`] is
/// returned. [`ProposalResult::Pending`] otherwise.
pub enum ProposalResult<P: ProposalT> {
  /// A [`Proposal`] that has yet to be executed.
  Pending(P),
  /// A [`Proposal`]'s execution output.
  Executed(P::Output),
}

/// A transaction to create a [`Proposal`].
#[derive(Debug)]
pub struct CreateProposal<'i, A> {
  identity: &'i mut OnChainIdentity,
  chained_execution: bool,
  ptb: ProgrammableTransaction,
  _action: PhantomData<A>,
}

impl<A> CreateProposal<'_, A> {
  /// Returns this [Transaction]'s [ProgrammableTransaction].
  pub fn ptb(&self) -> &ProgrammableTransaction {
    &self.ptb
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl<A> Transaction for CreateProposal<'_, A>
where
  Proposal<A>: ProposalT<Action = A> + DeserializeOwned,
  A: OptionalSend + OptionalSync,
{
  type Output = ProposalResult<Proposal<A>>;

  async fn build_programmable_transaction(
    &self,
    _client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    Ok(self.ptb.clone())
  }

  async fn apply(
    self,
    effects: &IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> Result<Self::Output, Error> {
    let Self {
      identity,
      chained_execution,
      ..
    } = self;

    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    // Identity has been changed regardless of whether the proposal has been executed
    // or simply created. Refetch it, to sync it with its on-chain state.
    *identity = get_identity(client, identity.id())
      .await?
      .expect("identity exists on-chain");

    if chained_execution {
      // The proposal has been created and executed right-away. Parse its effects.
      Proposal::<A>::parse_tx_effects(effects).map(ProposalResult::Executed)
    } else {
      // 2 objects are created, one is the Bag's Field and the other is our Proposal. Proposal is not owned by the bag,
      // but the field is.
      let proposals_bag_id = identity.multicontroller().proposals_bag_id();
      let proposal_id = effects
        .created()
        .iter()
        .find(|obj_ref| obj_ref.owner != proposals_bag_id)
        .expect("tx was successful")
        .object_id();

      client.get_object_by_id(proposal_id).await.map(ProposalResult::Pending)
    }
  }
}

/// A transaction to execute a [`Proposal`].
#[derive(Debug)]
pub struct ExecuteProposal<'i, A> {
  ptb: ProgrammableTransaction,
  identity: &'i mut OnChainIdentity,
  _action: PhantomData<A>,
}

impl<A> ExecuteProposal<'_, A> {
  /// Returns this [Transaction]'s [ProgrammableTransaction].
  pub fn ptb(&self) -> &ProgrammableTransaction {
    &self.ptb
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl<A> Transaction for ExecuteProposal<'_, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: OptionalSend + OptionalSync,
{
  type Output = <Proposal<A> as ProposalT>::Output;
  async fn build_programmable_transaction(
    &self,
    _client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    Ok(self.ptb.clone())
  }
  async fn apply(
    self,
    effects: &IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> Result<Self::Output, Error> {
    let Self { identity, .. } = self;

    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }
    *identity = get_identity(client, identity.id())
      .await?
      .expect("identity exists on-chain");

    Proposal::<A>::parse_tx_effects(effects)
  }
}

/// A transaction to approve a [`Proposal`].
#[derive(Debug)]
pub struct ApproveProposal<'p, 'i, A> {
  proposal: &'p mut Proposal<A>,
  identity: &'i OnChainIdentity,
  controller_token: ObjectID,
  cached_ptb: OnceCell<ProgrammableTransaction>,
}

impl<'p, 'i, A> ApproveProposal<'p, 'i, A> {
  /// Creates a new [Transaction] to approve `identity`'s `proposal`.
  pub fn new(
    proposal: &'p mut Proposal<A>,
    identity: &'i OnChainIdentity,
    controller_token: &ControllerToken,
  ) -> Result<Self, Error> {
    if identity.id() != controller_token.controller_of() {
      return Err(Error::Identity(format!(
        "token {} doesn't grant access to identity {}",
        controller_token.id(),
        identity.id()
      )));
    }

    Ok(Self {
      proposal,
      identity,
      controller_token: controller_token.id(),
      cached_ptb: OnceCell::new(),
    })
  }
}
impl<A: MoveType> ApproveProposal<'_, '_, A> {
  async fn make_ptb(&self, client: &IdentityClientReadOnly) -> Result<ProgrammableTransaction, Error> {
    let Self {
      proposal,
      identity,
      controller_token,
      ..
    } = self;
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .ok_or_else(|| Error::Identity(format!("identity {} doesn't exist", identity.id())))?;
    let controller_cap = client
      .get_object_ref_by_id(*controller_token)
      .await?
      .ok_or_else(|| Error::Identity(format!("controller token {} doesn't exist", controller_token)))?;
    let tx = <IdentityMoveCallsAdapter as IdentityMoveCalls>::approve_proposal::<A>(
      identity_ref.clone(),
      controller_cap.reference.to_object_ref(),
      proposal.id(),
      client.package_id(),
    )?;

    Ok(bcs::from_bytes(&tx)?)
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl<A> Transaction for ApproveProposal<'_, '_, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: MoveType + OptionalSend + OptionalSync,
{
  type Output = ();
  async fn build_programmable_transaction(
    &self,
    client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    self.cached_ptb.get_or_try_init(|| self.make_ptb(client)).await.cloned()
  }
  async fn apply(
    self,
    effects: &IotaTransactionBlockEffects,
    _client: &IdentityClientReadOnly,
  ) -> Result<Self::Output, Error> {
    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    let vp = self
      .identity
      .controller_voting_power(self.controller_token)
      .expect("is identity's controller");
    *self.proposal.votes_mut() = self.proposal.votes() + vp;

    Ok(())
  }
}

async fn obj_data_for_id(client: &IdentityClientReadOnly, obj_id: ObjectID) -> anyhow::Result<IotaObjectData> {
  use anyhow::Context;

  client
    .read_api()
    .get_object_with_options(obj_id, IotaObjectDataOptions::default().with_type().with_owner())
    .await?
    .into_object()
    .context("no iota object in response")
}

async fn obj_ref_and_type_for_id(
  client: &IdentityClientReadOnly,
  obj_id: ObjectID,
) -> anyhow::Result<(ObjectRef, TypeTag)> {
  let res = obj_data_for_id(client, obj_id).await?;
  let obj_ref = res.object_ref();
  let obj_type = match res.object_type().expect("object type is requested") {
    ObjectType::Package => anyhow::bail!("a move package cannot be sent"),
    ObjectType::Struct(type_) => type_.into(),
  };

  Ok((obj_ref, obj_type))
}

/// A transaction that requires user input in order to be executed.
pub struct UserDrivenTx<'i, A> {
  identity: &'i mut OnChainIdentity,
  controller_token: ObjectID,
  action: A,
  proposal_id: ObjectID,
  cached_ptb: OnceCell<ProgrammableTransaction>,
}

impl<'i, A> UserDrivenTx<'i, A> {
  fn new(identity: &'i mut OnChainIdentity, controller_token: ObjectID, action: A, proposal_id: ObjectID) -> Self {
    Self {
      identity,
      controller_token,
      action,
      proposal_id,
      cached_ptb: OnceCell::new(),
    }
  }
}
