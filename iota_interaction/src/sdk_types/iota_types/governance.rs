// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_types::Identifier;
use serde::{Deserialize, Serialize};

use super::balance::Balance;
use super::base_types::{EpochId, ObjectID};
use super::gas_coin::NANOS_PER_IOTA;
use super::id::{ID, UID};

/// Maximum number of active validators at any moment.
/// We do not allow the number of validators in any epoch to go above this.
pub const MAX_VALIDATOR_COUNT: u64 = 150;

/// Lower-bound on the amount of stake required to become a validator.
///
/// 2 million IOTA
pub const MIN_VALIDATOR_JOINING_STAKE_NANOS: u64 = 2_000_000 * NANOS_PER_IOTA;

/// Validators with stake amount below `validator_low_stake_threshold` are
/// considered to have low stake and will be escorted out of the validator set
/// after being below this threshold for more than
/// `validator_low_stake_grace_period` number of epochs.
///
/// 1.5 million IOTA
pub const VALIDATOR_LOW_STAKE_THRESHOLD_NANOS: u64 = 1_500_000 * NANOS_PER_IOTA;

/// Validators with stake below `validator_very_low_stake_threshold` will be
/// removed immediately at epoch change, no grace period.
///
/// 1 million IOTA
pub const VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NANOS: u64 = 1_000_000 * NANOS_PER_IOTA;

/// A validator can have stake below `validator_low_stake_threshold`
/// for this many epochs before being kicked out.
pub const VALIDATOR_LOW_STAKE_GRACE_PERIOD: u64 = 7;

pub const ADD_STAKE_MUL_COIN_FUN_NAME: Identifier =
    Identifier::from_static("request_add_stake_mul_coin");
pub const ADD_STAKE_FUN_NAME: Identifier = Identifier::from_static("request_add_stake");
pub const WITHDRAW_STAKE_FUN_NAME: Identifier = Identifier::from_static("request_withdraw_stake");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakedIota {
    id: UID,
    pool_id: ID,
    stake_activation_epoch: u64,
    principal: Balance,
}

impl StakedIota {
    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    pub fn pool_id(&self) -> ObjectID {
        self.pool_id.bytes
    }

    pub fn activation_epoch(&self) -> EpochId {
        self.stake_activation_epoch
    }

    pub fn request_epoch(&self) -> EpochId {
        // TODO: this might change when we implement warm up period.
        self.stake_activation_epoch.saturating_sub(1)
    }

    pub fn principal(&self) -> u64 {
        self.principal.value()
    }
}
