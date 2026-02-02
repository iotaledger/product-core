// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module tf_components::core_test_utils;

use iota::object::id_from_bytes;
use iota::vec_set::{Self, VecSet};
use std::string::String;

/// Simple Permission set for RoleMap tests
#[test_only]
public enum Permission has copy, drop, store {
    DeleteEverything,
    /// Manage Capabilities including adding and revoking
    ManageCapabilities,
    /// Manage Roles including adding, removing and updating
    ManageRoles,
    /// Some more actions
    ActionA,
    ActionB,
    ActionC,
}

public fun delete_everything(): Permission {
    Permission::DeleteEverything
}

public fun manage_capabilities(): Permission {
    Permission::ManageCapabilities
}

public fun manage_roles(): Permission {
    Permission::ManageRoles
}

public fun action_a(): Permission {
    Permission::ActionA
}

public fun action_b(): Permission {
    Permission::ActionB
}

public fun action_c(): Permission {
    Permission::ActionC
}

// --------------------------- Functions creating permission sets for often used roles ---------------------------

/// Create permissions typical used for the `super-admin` role permissions
public fun super_admin_permissions(): VecSet<Permission> {
    let mut perms = vec_set::empty();
    perms.insert(delete_everything());
    perms.insert(manage_capabilities());
    perms.insert(manage_roles());
    perms.insert(action_a());
    perms.insert(action_b());
    perms.insert(action_c());
    perms
}

public fun user_permissions(): VecSet<Permission> {
    let mut perms = vec_set::empty();
    perms.insert(action_a());
    perms.insert(action_b());
    perms.insert(action_c());
    perms
}

public fun fake_object_id_from_string(id_string: &String): ID {
    id_from_bytes(iota::hash::blake2b256(id_string.as_bytes()))
}

/// Create RoleAdminPermissions and CapabilityAdminPermissions with default test permissions
public fun get_admin_permissions(): (
    tf_components::role_map::RoleAdminPermissions<Permission>,
    tf_components::role_map::CapabilityAdminPermissions<Permission>,
) {
    let role_admin_permissions = tf_components::role_map::new_role_admin_permissions(
        manage_roles(),
        manage_roles(),
        manage_roles(),
    );
    let capability_admin_permissions = tf_components::role_map::new_capability_admin_permissions(
        manage_capabilities(),
        manage_capabilities(),
    );
    (role_admin_permissions, capability_admin_permissions)
}

const INITIAL_ADMIN_ROLE_NAME: vector<u8> = b"Admin";
const SECURITY_VAULT_ID_STRING: vector<u8> = b"TestVault";

public fun initial_admin_role_name(): String {
    INITIAL_ADMIN_ROLE_NAME.to_string()
}

/// Create a new RoleMap with an Admin capability for testing
/// Returns the RoleMap, admin capability, and the target_key
public fun create_test_role_map(
    ctx: &mut iota::tx_context::TxContext,
): (tf_components::role_map::RoleMap<Permission>, tf_components::capability::Capability, ID) {
    let target_key = fake_object_id_from_string(&SECURITY_VAULT_ID_STRING.to_string());
    let initial_admin_role = INITIAL_ADMIN_ROLE_NAME.to_string();
    let (role_admin_permissions, capability_admin_permissions) = get_admin_permissions();

    let (role_map, admin_cap) = tf_components::role_map::new(
        target_key,
        initial_admin_role,
        super_admin_permissions(),
        role_admin_permissions,
        capability_admin_permissions,
        ctx,
    );

    (role_map, admin_cap, target_key)
}

// --------------------------- Helper Fuctions -------------------------------------------------------------------
