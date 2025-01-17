// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::DerefMut as _;
use std::str::FromStr as _;

use crate::iota_interaction_adapter::AdapterError;
use crate::iota_interaction_adapter::AdapterNativeResponse;
use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdapter;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::OptionalSync;

use crate::rebased::client::IdentityClient;
use crate::rebased::migration::Proposal;
use async_trait::async_trait;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::collection_types::Entry;
use identity_iota_interaction::types::collection_types::VecMap;
use identity_iota_interaction::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::iota::types::Number;
use crate::rebased::migration::OnChainIdentity;
use crate::rebased::Error;
use identity_iota_interaction::MoveType;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::ProposalBuilder;
use super::ProposalT;

/// [`Proposal`] action that modifies an [`OnChainIdentity`]'s configuration - e.g:
/// - remove controllers
/// - add controllers
/// - update controllers voting powers
/// - update threshold
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(try_from = "Modify")]
pub struct ConfigChange {
  threshold: Option<u64>,
  controllers_to_add: HashMap<IotaAddress, u64>,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_voting_power: HashMap<ObjectID, u64>,
}

impl MoveType for ConfigChange {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::from_str(&format!("{package}::config_proposal::Modify")).expect("valid type tag")
  }
}

impl ProposalBuilder<'_, ConfigChange> {
  /// Sets a new value for the identity's threshold.
  pub fn threshold(mut self, threshold: u64) -> Self {
    self.set_threshold(threshold);
    self
  }

  /// Makes address `address` a new controller with voting power `voting_power`.
  pub fn add_controller(mut self, address: IotaAddress, voting_power: u64) -> Self {
    self.deref_mut().add_controller(address, voting_power);
    self
  }

  /// Adds multiple controllers. See [`ProposalBuilder::add_controller`].
  pub fn add_multiple_controllers<I>(mut self, controllers: I) -> Self
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    self.deref_mut().add_multiple_controllers(controllers);
    self
  }

  /// Removes an existing controller.
  pub fn remove_controller(mut self, controller_id: ObjectID) -> Self {
    self.deref_mut().remove_controller(controller_id);
    self
  }

  /// Removes many controllers.
  pub fn remove_multiple_controllers<I>(mut self, controllers: I) -> Self
  where
    I: IntoIterator<Item = ObjectID>,
  {
    self.deref_mut().remove_multiple_controllers(controllers);
    self
  }
}

impl ConfigChange {
  /// Creates a new [`ConfigChange`] proposal action.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the new threshold.
  pub fn set_threshold(&mut self, new_threshold: u64) {
    self.threshold = Some(new_threshold);
  }

  /// Adds a controller.
  pub fn add_controller(&mut self, address: IotaAddress, voting_power: u64) {
    self.controllers_to_add.insert(address, voting_power);
  }

  /// Adds many controllers.
  pub fn add_multiple_controllers<I>(&mut self, controllers: I)
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    for (addr, vp) in controllers {
      self.add_controller(addr, vp)
    }
  }

  /// Removes an existing controller.
  pub fn remove_controller(&mut self, controller_id: ObjectID) {
    self.controllers_to_remove.insert(controller_id);
  }

  /// Removes many controllers.
  pub fn remove_multiple_controllers<I>(&mut self, controllers: I)
  where
    I: IntoIterator<Item = ObjectID>,
  {
    for controller in controllers {
      self.remove_controller(controller)
    }
  }

  fn validate(&self, identity: &OnChainIdentity) -> Result<(), Error> {
    let new_threshold = self.threshold.unwrap_or(identity.threshold());
    let mut controllers = identity.controllers().clone();
    // check if update voting powers is valid
    for (controller, new_vp) in &self.controllers_voting_power {
      match controllers.get_mut(controller) {
        Some(vp) => *vp = *new_vp,
        None => {
          return Err(Error::InvalidConfig(format!(
            "object \"{controller}\" is not among identity \"{}\"'s controllers",
            identity.id()
          )))
        }
      }
    }
    // check if deleting controllers is valid
    for controller in &self.controllers_to_remove {
      if controllers.remove(controller).is_none() {
        return Err(Error::InvalidConfig(format!(
          "object \"{controller}\" is not among identity \"{}\"'s controllers",
          identity.id()
        )));
      }
    }
    // check if adding controllers is valid
    for (controller, vp) in &self.controllers_to_add {
      if controllers.insert((*controller).into(), *vp).is_some() {
        return Err(Error::InvalidConfig(format!(
          "object \"{controller}\" is already among identity \"{}\"'s controllers",
          identity.id()
        )));
      }
    }
    // check whether the new threshold allows to interact with the identity
    if new_threshold > controllers.values().sum() {
      return Err(Error::InvalidConfig(
        "the resulting configuration will result in an unaccessible identity".to_string(),
      ));
    }
    Ok(())
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProposalT for Proposal<ConfigChange> {
  type Action = ConfigChange;
  type Output = ();
  type Response = IotaTransactionBlockResponseAdapter;

  async fn create<'i, S>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    // Check the validity of the proposed changes.
    action.validate(identity)?;

    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;
    let sender_vp = identity
      .controller_voting_power(controller_cap_ref.0)
      .expect("controller exists");
    let chained_execution = sender_vp >= identity.threshold();
    let tx = IdentityMoveCallsAdapter::propose_config_change(
      identity_ref,
      controller_cap_ref,
      expiration,
      action.threshold,
      action.controllers_to_add,
      action.controllers_to_remove,
      action.controllers_voting_power,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      chained_execution,
      _action: PhantomData,
    })
  }

  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<ExecuteProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let proposal_id = self.id();
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    let tx = IdentityMoveCallsAdapter::execute_config_change(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData,
    })
  }

  fn parse_tx_effects_internal(
    _tx_response: &dyn IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = AdapterNativeResponse>,
  ) -> Result<Self::Output, Error> {
    Ok(())
  }
}

#[derive(Debug, Deserialize)]
struct Modify {
  threshold: Option<Number<u64>>,
  controllers_to_add: VecMap<IotaAddress, Number<u64>>,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_to_update: VecMap<ObjectID, Number<u64>>,
}

impl TryFrom<Modify> for ConfigChange {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(value: Modify) -> Result<Self, Self::Error> {
    let Modify {
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    } = value;
    let threshold = threshold.map(|num| num.try_into()).transpose()?;
    let controllers_to_add = controllers_to_add
      .contents
      .into_iter()
      .map(|Entry { key, value }| value.try_into().map(|n| (key, n)))
      .collect::<Result<_, _>>()?;
    let controllers_to_update = controllers_to_update
      .contents
      .into_iter()
      .map(|Entry { key, value }| value.try_into().map(|n| (key, n)))
      .collect::<Result<_, _>>()?;
    Ok(Self {
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_voting_power: controllers_to_update,
    })
  }
}
