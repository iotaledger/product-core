// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Role Based Capabilities for access control management
///
/// [!WARNING]
///
/// The current implementation of the `RoleMap` and `Capability` modules is a first iteration and is expected
/// to undergo significant changes in the future. The public interface is not yet stable, and we anticipate breaking changes
/// as we refine the design and implementation based on feedback and evolving requirements.
///
/// The final design and API of these modules will be released as part of the Audit Trail product, which will be
/// the first product to integrate these components.
module tf_components::capability;

use iota::clock::{Self, Clock};
use std::string::String;

// ===== Errors =====

#[error]
const EValidityPeriodInconsistent: vector<u8> =
    b"Validity period is inconsistent: valid_from must be before or equal to valid_until";

// ===== Core Structures =====

/// Capability granting role-based access to a managed onchain object (i.e. an audit trail).
public struct Capability has key, store {
    id: UID,
    /// The target_key of the RoleMap instance this capability applies to.
    target_key: ID,
    /// The role granted by this capability.
    role: String,
    /// For whom has this capability been issued (optional)
    issued_to: Option<address>,
    /// Optional validity period start timestamp (in milliseconds since Unix epoch).
    /// * The specified timestamp is included in the validity period
    /// * If None, the capability is valid from creation time
    valid_from: Option<u64>,
    /// Optional validity period end timestamp (in milliseconds since Unix epoch).
    /// Last point in time where the capability is valid.
    /// * The specified timestamp is included in the validity period
    /// * If None, the capability does not expire
    valid_until: Option<u64>,
}

/// Create a new capability with a specific role and all available optional restrictions
///
/// Parameters:
/// * role: The role granted by this capability
/// * target_key: The target_key of the RoleMap instance this capability applies to. Usually the ID of the managed onchain object (i.e. an audit trail).
/// * issued_to: Optional address restriction; if Some(address), the capability is bound to that specific address
/// * valid_from: Optional. First point in time where the capability is valid (in milliseconds since Unix epoch). If Some(ts), the capability is valid from that timestamp onwards (inclusive)
/// * valid_until: Optional. Last point in time where the capability is valid (in milliseconds since Unix epoch). If Some(ts), the capability is valid until that timestamp (inclusive)
/// * ctx: The transaction context
///
/// Returns: The newly created Capability
///
/// Errors:
/// * EValidityPeriodInconsistent: If both valid_from and valid_until are provided and valid_from > valid_until
public(package) fun new_capability(
    role: String,
    target_key: ID,
    issued_to: Option<address>,
    valid_from: Option<u64>,
    valid_until: Option<u64>,
    ctx: &mut TxContext,
): Capability {
    if (valid_from.is_some() && valid_until.is_some()) {
        let from = valid_from.borrow();
        let until = valid_until.borrow();
        assert!(*from <= *until, EValidityPeriodInconsistent);
    };
    Capability {
        id: object::new(ctx),
        role,
        target_key,
        issued_to,
        valid_from,
        valid_until,
    }
}

/// Get the capability's ID
public fun id(self: &Capability): ID {
    object::uid_to_inner(&self.id)
}

/// Get the capability's role
public fun role(self: &Capability): &String {
    &self.role
}

/// Get the capability's target_key
public fun target_key(self: &Capability): ID {
    self.target_key
}

/// Check if the capability has a specific role
public fun has_role(self: &Capability, role: &String): bool {
    &self.role == role
}

// Get the capability's issued_to address
public fun issued_to(self: &Capability): &Option<address> {
    &self.issued_to
}

// Get the capability's valid_from timestamp
public fun valid_from(self: &Capability): &Option<u64> {
    &self.valid_from
}

// Get the capability's valid_until timestamp
public fun valid_until(self: &Capability): &Option<u64> {
    &self.valid_until
}

// Check if the capability is currently valid for `clock::timestamp_ms(clock)`
public fun is_currently_valid(self: &Capability, clock: &Clock): bool {
    let current_ts = clock::timestamp_ms(clock);
    self.is_valid_for_timestamp(current_ts)
}

// Check if the capability is valid for a specific timestamp (in milliseconds since Unix epoch)
public fun is_valid_for_timestamp(self: &Capability, timestamp_ms: u64): bool {
    let valid_from_ok = if (self.valid_from.is_some()) {
        let from = self.valid_from.borrow();
        timestamp_ms >= *from
    } else {
        true
    };
    let valid_until_ok = if (self.valid_until.is_some()) {
        let until = self.valid_until.borrow();
        timestamp_ms <= *until
    } else {
        true
    };
    valid_from_ok && valid_until_ok
}

/// Destroy a capability
public(package) fun destroy(self: Capability) {
    let Capability {
        id,
        role: _,
        target_key: _,
        issued_to: _,
        valid_from: _,
        valid_until: _,
    } = self;
    object::delete(id);
}

#[test_only]
public fun destroy_for_testing(self: Capability) {
    destroy(self);
}
