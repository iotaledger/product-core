// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{self, Display, Formatter};
use std::vec::Vec;

use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::iota_object::IotaObjectRef;
use crate::iota_types::base_types::EpochId;
use crate::iota_types::digests::{TransactionDigest, TransactionEventsDigest};
use crate::iota_types::gas::GasCostSummary;
use crate::iota_types::storage::{DeleteKind, WriteKind};
use crate::rpc_types::IotaEvent;
use crate::types::base_types::{IotaAddress, ObjectID, ObjectRef, SequenceNumber};
use crate::types::execution_status::ExecutionStatus;
use crate::types::iota_serde::{BigInt, IotaTypeTag, SequenceNumber as AsSequenceNumber};
use crate::types::object::Owner;
use crate::types::quorum_driver_types::ExecuteTransactionRequestType;

/// BCS serialized IotaTransactionBlockEffects
pub type IotaTransactionBlockEffectsBcs = Vec<u8>;

/// BCS serialized IotaTransactionBlockEvents
pub type IotaTransactionBlockEventsBcs = Vec<u8>;

/// BCS serialized ObjectChange
pub type ObjectChangeBcs = Vec<u8>;

/// BCS serialized BalanceChange
pub type BalanceChangeBcs = Vec<u8>;

/// BCS serialized IotaTransactionBlockKind
pub type IotaTransactionBlockKindBcs = Vec<u8>;

pub type CheckpointSequenceNumber = u64;

/// An argument to a transaction in a programmable transaction block
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum IotaArgument {
  /// The gas coin. The gas coin can only be used by-ref, except for with
  /// `TransferObjects`, which can use it by-value.
  GasCoin,
  /// One of the input objects or primitive values (from
  /// `ProgrammableTransactionBlock` inputs)
  Input(u16),
  /// The result of another transaction (from `ProgrammableTransactionBlock`
  /// transactions)
  Result(u16),
  /// Like a `Result` but it accesses a nested result. Currently, the only
  /// usage of this is to access a value from a Move call with multiple
  /// return values.
  NestedResult(u16, u16),
}

impl Display for IotaArgument {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::GasCoin => write!(f, "GasCoin"),
      Self::Input(i) => write!(f, "Input({i})"),
      Self::Result(i) => write!(f, "Result({i})"),
      Self::NestedResult(i, j) => write!(f, "NestedResult({i},{j})"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "IotaExecutionResult", rename_all = "camelCase")]
pub struct IotaExecutionResult {
  /// The value of any arguments that were mutably borrowed.
  /// Non-mut borrowed values are not included
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub mutable_reference_outputs: Vec<(/* argument */ IotaArgument, Vec<u8>, IotaTypeTag)>,
  /// The return values from the transaction
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub return_values: Vec<(Vec<u8>, IotaTypeTag)>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(
    rename_all = "camelCase",
    rename = "TransactionBlockResponseOptions",
    default
)]
pub struct IotaTransactionBlockResponseOptions {
    /// Whether to show transaction input data. Default to be False
    pub show_input: bool,
    /// Whether to show bcs-encoded transaction input data
    pub show_raw_input: bool,
    /// Whether to show transaction effects. Default to be False
    pub show_effects: bool,
    /// Whether to show transaction events. Default to be False
    pub show_events: bool,
    /// Whether to show object_changes. Default to be False
    pub show_object_changes: bool,
    /// Whether to show balance_changes. Default to be False
    pub show_balance_changes: bool,
    /// Whether to show raw transaction effects. Default to be False
    pub show_raw_effects: bool,
}

impl IotaTransactionBlockResponseOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn full_content() -> Self {
        Self {
            show_effects: true,
            show_input: true,
            show_raw_input: true,
            show_events: true,
            show_object_changes: true,
            show_balance_changes: true,
            // This field is added for graphql execution. We keep it false here
            // so current users of `full_content` will not get raw effects unexpectedly.
            show_raw_effects: false,
        }
    }

    pub fn with_input(mut self) -> Self {
        self.show_input = true;
        self
    }

    pub fn with_raw_input(mut self) -> Self {
        self.show_raw_input = true;
        self
    }

    pub fn with_effects(mut self) -> Self {
        self.show_effects = true;
        self
    }

    pub fn with_events(mut self) -> Self {
        self.show_events = true;
        self
    }

    pub fn with_balance_changes(mut self) -> Self {
        self.show_balance_changes = true;
        self
    }

    pub fn with_object_changes(mut self) -> Self {
        self.show_object_changes = true;
        self
    }

    pub fn with_raw_effects(mut self) -> Self {
        self.show_raw_effects = true;
        self
    }

    /// default to return `WaitForEffectsCert` unless some options require
    /// local execution
    pub fn default_execution_request_type(&self) -> ExecuteTransactionRequestType {
        // if people want effects or events, they typically want to wait for local
        // execution
        if self.require_effects() {
            ExecuteTransactionRequestType::WaitForLocalExecution
        } else {
            ExecuteTransactionRequestType::WaitForEffectsCert
        }
    }

    pub fn require_input(&self) -> bool {
        self.show_input || self.show_raw_input || self.show_object_changes
    }

    pub fn require_effects(&self) -> bool {
        self.show_effects
            || self.show_events
            || self.show_balance_changes
            || self.show_object_changes
            || self.show_raw_effects
    }

    pub fn only_digest(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "ExecutionStatus", rename_all = "camelCase", tag = "status")]
pub enum IotaExecutionStatus {
    // Gas used in the success case.
    Success,
    // Gas used in the failed case, and the error.
    Failure { error: String },
}

impl Display for IotaExecutionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure { error } => write!(f, "failure due to {error}"),
        }
    }
}

impl IotaExecutionStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, IotaExecutionStatus::Success { .. })
    }
    pub fn is_err(&self) -> bool {
        matches!(self, IotaExecutionStatus::Failure { .. })
    }
}

impl From<ExecutionStatus> for IotaExecutionStatus {
    fn from(status: ExecutionStatus) -> Self {
        match status {
            ExecutionStatus::Success => Self::Success,
            ExecutionStatus::Failure {
                error,
                command: None,
            } => Self::Failure {
                error: format!("{error:?}"),
            },
            ExecutionStatus::Failure {
                error,
                command: Some(idx),
            } => Self::Failure {
                error: format!("{error:?} in command {idx}"),
            },
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "OwnedObjectRef")]
pub struct OwnedObjectRef {
    pub owner: Owner,
    pub reference: IotaObjectRef,
}

impl OwnedObjectRef {
    pub fn object_id(&self) -> ObjectID {
        self.reference.object_id
    }
    pub fn version(&self) -> SequenceNumber {
        self.reference.version
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[enum_dispatch(IotaTransactionBlockEffectsAPI)]
#[serde(
    rename = "TransactionBlockEffects",
    rename_all = "camelCase",
    tag = "messageVersion"
)]
pub enum IotaTransactionBlockEffects {
    V1(IotaTransactionBlockEffectsV1),
}

#[enum_dispatch]
pub trait IotaTransactionBlockEffectsAPI {
    fn status(&self) -> &IotaExecutionStatus;
    fn into_status(self) -> IotaExecutionStatus;
    fn shared_objects(&self) -> &[IotaObjectRef];
    fn created(&self) -> &[OwnedObjectRef];
    fn mutated(&self) -> &[OwnedObjectRef];
    fn unwrapped(&self) -> &[OwnedObjectRef];
    fn deleted(&self) -> &[IotaObjectRef];
    fn unwrapped_then_deleted(&self) -> &[IotaObjectRef];
    fn wrapped(&self) -> &[IotaObjectRef];
    fn gas_object(&self) -> &OwnedObjectRef;
    fn events_digest(&self) -> Option<&TransactionEventsDigest>;
    fn dependencies(&self) -> &[TransactionDigest];
    fn executed_epoch(&self) -> EpochId;
    fn transaction_digest(&self) -> &TransactionDigest;
    fn gas_cost_summary(&self) -> &GasCostSummary;

    /// Return an iterator of mutated objects, but excluding the gas object.
    fn mutated_excluding_gas(&self) -> Vec<OwnedObjectRef>;
    fn modified_at_versions(&self) -> Vec<(ObjectID, SequenceNumber)>;
    fn all_changed_objects(&self) -> Vec<(&OwnedObjectRef, WriteKind)>;
    fn all_deleted_objects(&self) -> Vec<(&IotaObjectRef, DeleteKind)>;
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(
    rename = "TransactionBlockEffectsModifiedAtVersions",
    rename_all = "camelCase"
)]
pub struct IotaTransactionBlockEffectsModifiedAtVersions {
    object_id: ObjectID,
    #[schemars(with = "AsSequenceNumber")]
    #[serde_as(as = "AsSequenceNumber")]
    sequence_number: SequenceNumber,
}

/// The response from processing a transaction or a certified transaction
#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionBlockEffectsV1", rename_all = "camelCase")]
pub struct IotaTransactionBlockEffectsV1 {
    /// The status of the execution
    pub status: IotaExecutionStatus,
    /// The epoch when this transaction was executed.
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub executed_epoch: EpochId,
    pub gas_used: GasCostSummary,
    /// The version that every modified (mutated or deleted) object had before
    /// it was modified by this transaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified_at_versions: Vec<IotaTransactionBlockEffectsModifiedAtVersions>,
    /// The object references of the shared objects used in this transaction.
    /// Empty if no shared objects were used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_objects: Vec<IotaObjectRef>,
    /// The transaction digest
    pub transaction_digest: TransactionDigest,
    /// ObjectRef and owner of new objects created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of mutated objects, including gas object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutated: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in
    /// the past, and just got extracted out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped: Vec<OwnedObjectRef>,
    /// Object Refs of objects now deleted (the old refs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<IotaObjectRef>,
    /// Object refs of objects previously wrapped in other objects but now
    /// deleted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped_then_deleted: Vec<IotaObjectRef>,
    /// Object refs of objects now wrapped in other objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wrapped: Vec<IotaObjectRef>,
    /// The updated gas object reference. Have a dedicated field for convenient
    /// access. It's also included in mutated.
    pub gas_object: OwnedObjectRef,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<TransactionDigest>,
}

impl IotaTransactionBlockEffectsAPI for IotaTransactionBlockEffectsV1 {
    fn status(&self) -> &IotaExecutionStatus {
        &self.status
    }
    fn into_status(self) -> IotaExecutionStatus {
        self.status
    }
    fn shared_objects(&self) -> &[IotaObjectRef] {
        &self.shared_objects
    }
    fn created(&self) -> &[OwnedObjectRef] {
        &self.created
    }
    fn mutated(&self) -> &[OwnedObjectRef] {
        &self.mutated
    }
    fn unwrapped(&self) -> &[OwnedObjectRef] {
        &self.unwrapped
    }
    fn deleted(&self) -> &[IotaObjectRef] {
        &self.deleted
    }
    fn unwrapped_then_deleted(&self) -> &[IotaObjectRef] {
        &self.unwrapped_then_deleted
    }
    fn wrapped(&self) -> &[IotaObjectRef] {
        &self.wrapped
    }
    fn gas_object(&self) -> &OwnedObjectRef {
        &self.gas_object
    }
    fn events_digest(&self) -> Option<&TransactionEventsDigest> {
        self.events_digest.as_ref()
    }
    fn dependencies(&self) -> &[TransactionDigest] {
        &self.dependencies
    }

    fn executed_epoch(&self) -> EpochId {
        self.executed_epoch
    }

    fn transaction_digest(&self) -> &TransactionDigest {
        &self.transaction_digest
    }

    fn gas_cost_summary(&self) -> &GasCostSummary {
        &self.gas_used
    }

    fn mutated_excluding_gas(&self) -> Vec<OwnedObjectRef> {
        self.mutated
            .iter()
            .filter(|o| *o != &self.gas_object)
            .cloned()
            .collect()
    }

    fn modified_at_versions(&self) -> Vec<(ObjectID, SequenceNumber)> {
        self.modified_at_versions
            .iter()
            .map(|v| (v.object_id, v.sequence_number))
            .collect::<Vec<_>>()
    }

    fn all_changed_objects(&self) -> Vec<(&OwnedObjectRef, WriteKind)> {
        self.mutated
            .iter()
            .map(|owner_ref| (owner_ref, WriteKind::Mutate))
            .chain(
                self.created
                    .iter()
                    .map(|owner_ref| (owner_ref, WriteKind::Create)),
            )
            .chain(
                self.unwrapped
                    .iter()
                    .map(|owner_ref| (owner_ref, WriteKind::Unwrap)),
            )
            .collect()
    }

    fn all_deleted_objects(&self) -> Vec<(&IotaObjectRef, DeleteKind)> {
        self.deleted
            .iter()
            .map(|r| (r, DeleteKind::Normal))
            .chain(
                self.unwrapped_then_deleted
                    .iter()
                    .map(|r| (r, DeleteKind::UnwrapThenDelete)),
            )
            .chain(self.wrapped.iter().map(|r| (r, DeleteKind::Wrap)))
            .collect()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename = "TransactionBlockEvents", transparent)]
pub struct IotaTransactionBlockEvents {
  pub data: Vec<IotaEvent>,
}

/// The response from processing a dev inspect transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "DevInspectResults", rename_all = "camelCase")]
pub struct DevInspectResults {
  /// Summary of effects that likely would be generated if the transaction is
  /// actually run. Note however, that not all dev-inspect transactions
  /// are actually usable as transactions, so it might not be possible
  /// actually generate these effects from a normal transaction.
  pub effects: IotaTransactionBlockEffects,
  /// Events that likely would be generated if the transaction is actually
  /// run.
  pub events: IotaTransactionBlockEvents,
  /// Execution results (including return values) from executing the
  /// transactions
  #[serde(skip_serializing_if = "Option::is_none")]
  pub results: Option<Vec<IotaExecutionResult>>,
  /// Execution error from executing the transactions
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
  /// The raw transaction data that was dev inspected.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub raw_txn_data: Vec<u8>,
  /// The raw effects of the transaction that was dev inspected.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub raw_effects: Vec<u8>,
}

// TODO: this file might not be the best place for this struct.
/// Additional arguments supplied to dev inspect beyond what is allowed in
/// today's API.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "DevInspectArgs", rename_all = "camelCase")]
pub struct DevInspectArgs {
  /// The sponsor of the gas for the transaction might be different from the
  /// sender.
  pub gas_sponsor: Option<IotaAddress>,
  /// The gas budget for the transaction.
  pub gas_budget: Option<BigInt<u64>>,
  /// The gas objects used to pay for the transaction.
  pub gas_objects: Option<Vec<ObjectRef>>,
  /// Whether to skip transaction checks for the transaction.
  pub skip_checks: Option<bool>,
  /// Whether to return the raw transaction data and effects.
  pub show_raw_txn_data_and_effects: Option<bool>,
}
