// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::vec::Vec;

pub use iota_sdk_types::{
    Argument, ChangeEpoch, ChangeEpochV2, ChangeEpochV3, ChangeEpochV4, Command,
    EndOfEpochTransactionKind, GasPayment as GasData, GenesisObject, GenesisTransaction,
    MakeMoveVector, MergeCoins, MoveCall as ProgrammableMoveCall, ProgrammableTransaction, Publish,
    RandomnessStateUpdate, SharedObjectReference as SharedObjectRef, SplitCoins, SystemPackage,
    Transaction as TransactionData, TransactionExpiration, TransactionKind,
    TransactionV1 as TransactionDataV1, TransferObjects, Upgrade,
};
use iota_sdk_types::Input;
use nonempty::{NonEmpty, nonempty};
use serde::{Deserialize, Serialize};
use crate::{fp_ensure, fp_bail};

use super::base_types::{IotaAddress, ObjectID, ObjectRef, SequenceNumber};
use super::error::{UserInputError, UserInputResult};

pub const TEST_ONLY_GAS_UNIT_FOR_TRANSFER: u64 = 10_000;
pub const TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_PUBLISH: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_STAKING: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_GENERIC: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN: u64 = 10_000;
// For some transactions we may either perform heavy operations or touch
// objects that are storage expensive. That may happen (and often is the case)
// because the object touched are set up in genesis and carry no storage cost
// (and thus rebate) on first usage.
pub const TEST_ONLY_GAS_UNIT_FOR_HEAVY_COMPUTATION_STORAGE: u64 = 5_000_000;

pub const GAS_PRICE_FOR_SYSTEM_TX: u64 = 1;

pub const DEFAULT_VALIDATOR_GAS_PRICE: u64 = 1000;

/// Type alias for the SDK's `Input` type, used as transaction call arguments.
pub type CallArg = Input;

/// API for accessing and constructing [`TransactionData`].
///
/// This trait provides node-internal methods for:
/// - **Accessors**: reading transaction fields (sender, kind, gas, expiration,
///   etc.)
/// - **Queries**: inspecting transaction properties (shared objects, Move
///   calls, sponsorship)
/// - **Validation**: checking transaction validity against protocol config
/// - **Constructors**: building new transactions (transfers, Move calls,
///   programmable txs, etc.)
///
/// Note: The `iota-rust-sdk` crate (`iota-sdk-types`) defines its own
/// [`Transaction`] type with additional client-facing methods.
pub trait TransactionDataAPI {
    /// Returns the address of the transaction sender.
    fn sender(&self) -> IotaAddress;

    /// Returns a reference to the transaction kind.
    fn kind(&self) -> &TransactionKind;

    /// Returns a mutable reference to the transaction kind.
    fn kind_mut(&mut self) -> &mut TransactionKind;

    /// Consumes self and returns the transaction kind.
    fn into_kind(self) -> TransactionKind;

    /// Returns the transaction signer(s). Includes both the sender and the gas
    /// owner if they differ (i.e. for sponsored transactions).
    fn signers(&self) -> NonEmpty<IotaAddress>;

    /// Returns a reference to the gas data (owner, payment objects, price,
    /// budget).
    fn gas_data(&self) -> &GasData;

    /// Returns the address that owns the gas payment objects.
    fn gas_owner(&self) -> IotaAddress;

    /// Returns the gas payment object references.
    fn gas(&self) -> &[ObjectRef];

    /// Returns the gas price for this transaction.
    fn gas_price(&self) -> u64;

    /// Returns the gas budget for this transaction.
    fn gas_budget(&self) -> u64;

    /// Returns the transaction expiration.
    fn expiration(&self) -> &TransactionExpiration;

    fn gas_data_mut(&mut self) -> &mut GasData;
}

impl TransactionDataAPI for TransactionData {
    fn sender(&self) -> IotaAddress {
        match self {
            Self::V1(v1) => v1.sender,
            _ => unimplemented!("a new Transaction enum variant was added and needs to be handled"),
        }
    }

    fn kind(&self) -> &TransactionKind {
        match self {
            Self::V1(v1) => &v1.kind,
            _ => unimplemented!("a new Transaction enum variant was added and needs to be handled"),
        }
    }

    fn kind_mut(&mut self) -> &mut TransactionKind {
        match self {
            Self::V1(v1) => &mut v1.kind,
            _ => unimplemented!("a new Transaction enum variant was added and needs to be handled"),
        }
    }

    fn into_kind(self) -> TransactionKind {
        match self {
            Self::V1(v1) => v1.kind,
            _ => unimplemented!("a new Transaction enum variant was added and needs to be handled"),
        }
    }

    fn signers(&self) -> NonEmpty<IotaAddress> {
        let mut signers = nonempty![self.sender()];
        if self.gas_owner() != self.sender() {
            signers.push(self.gas_owner());
        }
        signers
    }

    fn gas_data(&self) -> &GasData {
        match self {
            Self::V1(v1) => &v1.gas_payment,
            _ => unimplemented!("a new Transaction enum variant was added and needs to be handled"),
        }
    }

    fn gas_owner(&self) -> IotaAddress {
        self.gas_data().owner
    }

    fn gas(&self) -> &[ObjectRef] {
        &self.gas_data().objects
    }

    fn gas_price(&self) -> u64 {
        self.gas_data().price
    }

    fn gas_budget(&self) -> u64 {
        self.gas_data().budget
    }

    fn expiration(&self) -> &TransactionExpiration {
        match self {
            Self::V1(v1) => &v1.expiration,
            _ => unimplemented!("a new Transaction variant was added and needs to be handled"),
        }
    }

    fn gas_data_mut(&mut self) -> &mut GasData {
        match self {
            Self::V1(v1) => &mut v1.gas_payment,
            _ => unimplemented!("a new Transaction variant was added and needs to be handled"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum InputObjectKind {
    // A Move package, must be immutable.
    MovePackage(ObjectID),
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedMoveObject(ObjectRef),
    // A Move object that's shared and mutable.
    SharedMoveObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    },
}

impl InputObjectKind {
    pub fn object_id(&self) -> ObjectID {
        match self {
            Self::MovePackage(id) => *id,
            Self::ImmOrOwnedMoveObject(object_ref) => object_ref.object_id,
            Self::SharedMoveObject { id, .. } => *id,
        }
    }

    pub fn version(&self) -> Option<SequenceNumber> {
        match self {
            Self::MovePackage(..) => None,
            Self::ImmOrOwnedMoveObject(object_ref) => Some(object_ref.version),
            Self::SharedMoveObject { .. } => None,
        }
    }

    pub fn object_not_found_error(&self) -> UserInputError {
        match *self {
            Self::MovePackage(package_id) => {
                UserInputError::DependentPackageNotFound { package_id }
            }
            Self::ImmOrOwnedMoveObject(object_ref) => UserInputError::ObjectNotFound {
                object_id: object_ref.object_id,
                version: Some(object_ref.version),
            },
            Self::SharedMoveObject { id, .. } => UserInputError::ObjectNotFound {
                object_id: id,
                version: None,
            },
        }
    }

    pub fn is_shared_object(&self) -> bool {
        matches!(self, Self::SharedMoveObject { .. })
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            Self::MovePackage(..) => false,
            Self::ImmOrOwnedMoveObject(_) => true,
            Self::SharedMoveObject { mutable, .. } => *mutable,
        }
    }

    /// Merges another InputObjectKind into self.
    ///
    /// For shared objects, if either is mutable, the result is mutable. Fails
    /// if the IDs or initial versions do not match.
    /// For non-shared objects, fails if they are not equal.
    pub fn left_union_with_checks(&mut self, other: &InputObjectKind) -> UserInputResult<()> {
        match self {
            InputObjectKind::MovePackage(_) | InputObjectKind::ImmOrOwnedMoveObject(_) => {
                fp_ensure!(
                    self == other,
                    UserInputError::InconsistentInput {
                        object_id: other.object_id(),
                    }
                );
            }
            InputObjectKind::SharedMoveObject {
                id,
                initial_shared_version,
                mutable,
            } => match other {
                InputObjectKind::MovePackage(_) | InputObjectKind::ImmOrOwnedMoveObject(_) => {
                    fp_bail!(UserInputError::NotSharedObject)
                }
                InputObjectKind::SharedMoveObject {
                    id: other_id,
                    initial_shared_version: other_initial_shared_version,
                    mutable: other_mutable,
                } => {
                    fp_ensure!(id == other_id, UserInputError::SharedObjectIdMismatch);
                    fp_ensure!(
                        initial_shared_version == other_initial_shared_version,
                        UserInputError::SharedObjectStartingVersionMismatch
                    );

                    if !*mutable && *other_mutable {
                        *mutable = *other_mutable;
                    }
                }
            },
        }

        Ok(())
    }

    /// Checks that `self` and `other` are equal for non-shared objects.
    /// For shared objects, checks that IDs and initial versions match while
    /// mutability can be different.
    pub fn check_consistency_for_authentication(
        &self,
        other: &InputObjectKind,
    ) -> UserInputResult<()> {
        match self {
            InputObjectKind::MovePackage(_) | InputObjectKind::ImmOrOwnedMoveObject(_) => {
                fp_ensure!(
                    self == other,
                    UserInputError::InconsistentInput {
                        object_id: self.object_id()
                    }
                );
            }
            InputObjectKind::SharedMoveObject {
                id,
                initial_shared_version,
                mutable: _,
            } => match other {
                InputObjectKind::MovePackage(_) | InputObjectKind::ImmOrOwnedMoveObject(_) => {
                    fp_bail!(UserInputError::InconsistentInput {
                        object_id: self.object_id()
                    })
                }
                InputObjectKind::SharedMoveObject {
                    id: other_id,
                    initial_shared_version: other_initial_shared_version,
                    mutable: _,
                } => {
                    fp_ensure!(
                        id == other_id,
                        UserInputError::InconsistentInput { object_id: *id }
                    );
                    fp_ensure!(
                        initial_shared_version == other_initial_shared_version,
                        UserInputError::InconsistentInput { object_id: *id }
                    );
                }
            },
        }

        Ok(())
    }
}