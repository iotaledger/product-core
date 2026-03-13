// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Simple shared counter to demonstrate role_map::RoleMap integration
/// It also demonstrates how to use the generic role-data argument of the RoleMap
/// to implement time-based permissions by storing a weekday in the role data
/// and checking if the current day matches the stored weekday in the permission check.
#[test_only]
module tf_components::counter;

use iota::clock::Clock;
use tf_components::capability::Capability;
use tf_components::counter_permission::{Self as permission, CounterPermission};
use tf_components::role_map;

#[error]
const EWeekDayMismatch: vector<u8> =
    b"The role associated with the provided capability is restricted to a specific weekday which does not match the current weekday";

public enum Weekday has copy, drop, store {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

public fun weekday_to_u8(self: &Weekday): u8 {
    match (self) {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}

public fun monday(): Weekday {
    Weekday::Monday
}

public fun tuesday(): Weekday {
    Weekday::Tuesday
}

public fun wednesday(): Weekday {
    Weekday::Wednesday
}

public fun thursday(): Weekday {
    Weekday::Thursday
}

public fun friday(): Weekday {
    Weekday::Friday
}

public fun saturday(): Weekday {
    Weekday::Saturday
}

public fun sunday(): Weekday {
    Weekday::Sunday
}

public struct Counter has key {
    id: UID,
    value: u64,
    access: role_map::RoleMap<CounterPermission, Weekday>,
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

    // Create a `RoleMapAdminPermissions` instance to configure the permissions
    // that will be needed by users to administer the `RoleMap` itself.
    // There is only one action that needs to be configured with a permission of your choice:
    // * `migrate`: Permission required to migrate the `RoleMap` in case of an upgraded counter Move package
    let role_map_admin_permissions = role_map::new_role_map_admin_permissions(
        permission::migrate_counter(),
    );

    let (access, admin_cap) = role_map::new(
        counter_id,
        b"super-admin".to_string(),
        permission::super_admin_permissions(),
        role_admin_permissions,
        capability_admin_permissions,
        role_map_admin_permissions,
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

public fun increment(self: &mut Counter, cap: &Capability, clock: &Clock, ctx: &TxContext) {
    self.assert_capability_valid(
        cap,
        &permission::increment_counter(),
        clock,
        ctx,
    );
    self.value = self.value + 1;
}

public fun assert_capability_valid(
    self: &Counter,
    cap: &Capability,
    permission: &CounterPermission,
    clock: &Clock,
    ctx: &TxContext,
): bool {
    self.access.assert_capability_valid(
        cap,
        permission,
        clock,
        ctx
    );
    let role_data_option = self.access.get_role_data(cap.role());
    if (role_data_option.is_some_and!(|required_weekday| {
            let current_weekday = to_weekday(clock);
            weekday_to_u8(required_weekday) != current_weekday
        })) {
        assert!(false, EWeekDayMismatch);
    };
    true
}

public fun access(self: &Counter): &role_map::RoleMap<CounterPermission, Weekday> {
    &self.access
}

public fun access_mut(self: &mut Counter): &mut role_map::RoleMap<CounterPermission, Weekday> {
    &mut self.access
}

public fun value(self: &Counter): u64 {
    self.value
}

/// Returns the day of the week (0 = Monday, 1 = Tuesday, ..., 6 = Sunday)
/// based on a millisecond Unix timestamp.
/// The Unix epoch (timestamp 0) was a Thursday (day index 3).
public fun to_weekday(clock: &Clock): u8 {
    let timestamp_ms = clock.timestamp_ms();
    let ms_per_day: u64 = 86_400_000; // 24 * 60 * 60 * 1000
    let day_count = timestamp_ms / ms_per_day;
    // Unix epoch is Thursday. If Monday = 0, then Thursday = 3.
    // So we add 3 to shift the epoch day to Thursday, then mod 7.
    let weekday = ((day_count + 3) % 7) as u8;
    weekday
}
