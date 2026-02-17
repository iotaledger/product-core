// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Simple shared counter to demonstrate role_mao::RoleMap integration
#[test_only]
module tf_components::counter;

use iota::clock::Clock;
use tf_components::{
    capability::Capability,
    counter_permission::{Self as permission, CounterPermission},
    role_map
};

#[error]
const EPermissionDenied: vector<u8> =
    b"The role associated with the provided capability does not have the required permission";

public struct Counter has key {
    id: UID,
    value: u64,
    access: role_map::RoleMap<CounterPermission>,
}

public fun create(ctx: &mut TxContext): (Capability, ID) {
    let counter_uid = object::new(ctx);
    let counter_id = object::uid_to_inner(&counter_uid);

    // Create a `CapabilityAdminPermissions` instance to configure the permissions
    // that will be needed by users to issue and revoke capabilities with the `RoleMap`.
    //
    // There are two actions that need to be configured with a permission of your choice:
    // * `add`: Permission required to add (issue) a new capability
    // * `revoke`: Permission required to revoke an existing capability
    //
    // In this example we use a specific permission for each action.
    //
    let capability_admin_permissions = role_map::new_capability_admin_permissions(
        permission::add_capabilities(),
        permission::revoke_capabilities(),
    );

    // Create a `RoleAdminPermissions` instance to configure the permissions
    // that will be needed by users to administer roles with the `RoleMap`.
    //
    // There are three actions that need to be configured with a permission of your choice:
    // * `add`: Permission required to add a new role
    // * `delete`: Permission required to delete an existing role
    // * `update` Permission required to update permissions associated with an existing role
    //
    // In this example we allow to use all three actions with the `ManageRoles` permission
    // for the sake of simplicity. In a real world application you would probably have action
    // specific permissions like `AddRoles`, `DeleteRoles` and `UpdateRoles` like we did
    // above to specify the `CapabilityAdminPermissions`.
    //
    let role_admin_permissions = role_map::new_role_admin_permissions(
        permission::manage_roles(),
        permission::manage_roles(),
        permission::manage_roles(),
    );

    let (access, admin_cap) = role_map::new(
        counter_id,
        b"super-admin".to_string(),
        permission::super_admin_permissions(),
        role_admin_permissions,
        capability_admin_permissions,
        ctx,
    );

    let counter = Counter {
        id: counter_uid,
        value: 0,
        access,
    };
    transfer::share_object(counter);

    (admin_cap, counter_id)
}

public fun increment(counter: &mut Counter, cap: &Capability, clock: &Clock, ctx: &mut TxContext) {
    assert!(
        counter
            .access
            .is_capability_valid(
                cap,
                &permission::increment_counter(),
                clock,
                ctx,
            ),
        EPermissionDenied,
    );
    counter.value = counter.value + 1;
}

public fun access(counter: &Counter): &role_map::RoleMap<CounterPermission> {
    &counter.access
}

public fun access_mut(counter: &mut Counter): &mut role_map::RoleMap<CounterPermission> {
    &mut counter.access
}

public fun value(counter: &Counter): u64 {
    counter.value
}
