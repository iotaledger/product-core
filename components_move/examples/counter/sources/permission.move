// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Simple example Permissions for a shared counter
#[test_only]
module tf_components::counter_permission;

use iota::vec_set::{Self, VecSet};

/// Permissions for a shared counter
public enum CounterPermission has copy, drop, store {
    // --- Used for a super-admin role who can do everything ---
    /// Destroy the Counter
    DeleteCounter,
    /// Manage Capabilities: Adding new capabilities
    AddCapabilities,
    /// Manage Capabilities: Revoking existing capabilities
    RevokeCapabilities,
    /// One permission for the complete roles management, including adding, removing and updating roles
    ManageRoles,
    // --- Counter Management, could be used for a counter admin role---
    /// Increment the Counter
    IncrementCounter,
    /// Reset the Counter
    ResetCounter,
}

public fun delete_counter(): CounterPermission {
    CounterPermission::DeleteCounter
}

public fun add_capabilities(): CounterPermission {
    CounterPermission::AddCapabilities
}

public fun revoke_capabilities(): CounterPermission {
    CounterPermission::RevokeCapabilities
}

public fun manage_roles(): CounterPermission {
    CounterPermission::ManageRoles
}

public fun increment_counter(): CounterPermission {
    CounterPermission::IncrementCounter
}

public fun reset_counter(): CounterPermission {
    CounterPermission::ResetCounter
}

// --------------------------- Functions creating permission sets for often used roles ---------------------------

/// Create permissions typical used for the `super-admin` role permissions
public fun super_admin_permissions(): VecSet<CounterPermission> {
    let mut perms = vec_set::empty();
    perms.insert(delete_counter());
    perms.insert(add_capabilities());
    perms.insert(revoke_capabilities());
    perms.insert(manage_roles());
    perms.insert(increment_counter());
    perms.insert(reset_counter());
    perms
}

public fun counter_admin_permissions(): VecSet<CounterPermission> {
    let mut perms = vec_set::empty();
    perms.insert(increment_counter());
    perms.insert(reset_counter());
    perms
}
