// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use std::collections::HashSet;

use super::TransactionBuilderAdapter;
use crate::error::TsSdkError;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::transaction::Argument;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::BorrowIntentFnInternalT;
use identity_iota_interaction::ControllerIntentFnInternalT;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::MoveType;
use identity_iota_interaction::ProgrammableTransactionBcs;

pub struct IdentityMoveCallsTsSdk {}

impl IdentityMoveCalls for IdentityMoveCallsTsSdk {
  type Error = TsSdkError;
  type NativeTxBuilder = (); // TODO: Set this to the wasm32... type that is wrapped by IdentityMoveCallsTsSdk

  fn propose_borrow(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    objects: Vec<ObjectID>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn execute_borrow<F: BorrowIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn create_and_execute_borrow<F: BorrowIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn propose_config_change<I1, I2>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    expiration: Option<u64>,
    threshold: Option<u64>,
    controllers_to_add: I1,
    controllers_to_remove: HashSet<ObjectID>,
    controllers_to_update: I2,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    I1: IntoIterator<Item = (IotaAddress, u64)>,
    I2: IntoIterator<Item = (ObjectID, u64)>,
  {
    unimplemented!();
  }

  fn execute_config_change(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn propose_controller_execution(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    controller_cap_id: ObjectID,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn execute_controller_execution<F: ControllerIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn create_and_execute_controller_execution<F>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    F: ControllerIntentFnInternalT<Self::NativeTxBuilder>,
  {
    todo!()
  }

  fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn new_with_controllers<C>(
    did_doc: &[u8],
    controllers: C,
    threshold: u64,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn propose_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn execute_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn approve_proposal<T: MoveType>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn propose_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn create_and_execute_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn execute_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn propose_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    did_doc: impl AsRef<[u8]>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn execute_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn propose_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn execute_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }
}
