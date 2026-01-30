# Trust Framework Components

The tf_components package contains Move modules providing reusable components for building smart contracts
within the IOTA Trust Framework. These components are designed to be modular and easily integrated into various
Trust Framework products and community-developed smart contracts.

Modules Overview:
* role_map:   Implements the `RoleMap` struct for role-based access control, allowing mapping of roles to 
              application specific permissions.
* capability: Defines the `Capability` struct for granting access rights within smart contracts in conjunction with 
              the `RoleMap<P>` struct.
* timelock:   Provides the `Timelock` struct for time-based access control, enabling restrictions based on time 
              constraints.

## Role-Based Access Control - The `role_map` Module

The `role_map` module provides the `RoleMap<P>` struct and associated functions:
```
/// A role-based access control helper mapping unique role identifiers to their associated permissions.
///
/// Provides the following functionalities:
/// - Define an initial role with a custom set of permissions (i.e. an Admin role).
/// - Use custom permission types defined by the integrating module using the generic parameter `P`.
/// - Create, delete, and update roles and their permissions
/// - Issue, revoke, and destroy `tf_components::capability`s associated with a specific role.
/// - Validate `tf_components::capability`s against the defined roles to facilitate proper access control by other modules
///   (function `RoleMap.is_capability_valid()`)
/// - All functions are access restricted by custom permissions defined during `RoleMap` instantiation.
``` 
* The TF product Audit Trails uses the `RoleMap` to manage access to the audit trail records and their operations, which 
  can be seen as a more complex example:
  * The `RoleMap` is integrated in the `audit_trail::main` module to manage access to the audit trail records and
    their operations. See [here](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L208) for an example.
  * The `RoleMap` is created by the `AuditTrail` in it's [create function](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L114).
  * An example for the Move user experience can be found in the [capability_tests.move](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/tests/capability_tests.move) file.
* `RoleMap` directly depends on the `tf_components::capability::Capability` module. Both modules are tight strongly together.

### RoleMap Integration Example

To show how the `RoleMap` can be integrated into 3rd party shared objects (or TF products),
this example integrates the `RoleMap` into a simple shared `Counter` object, as being described
[here](https://docs.iota.org/developer/iota-101/move-overview/package-upgrades/upgrade#4-guard-function-access).

In general, to integrate the `RoleMap` into a shared object,
we need to define a Permission enum similar to the [enum used for IOTA Audit Trails](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/permission.move#L10-L11).

For example the Permission enum for the shared `Counter` could look like this:

In the file `counter/permission.move`:
```Move
/// Simple example Permissions for a shared counter
module counter::permission;

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
```

The `Counter` object then would need to instantiate the `role_map::RoleMap<CounterPermission>` in its create function like this:

In the file `counter/counter.move`:
```Move

#[error]
const EPermissionDenied: vector<u8> = b"The role associated with the provided capability does not have the required permission";

public struct Counter has key {
    id: UID,
    value: u64,
    roles: role_map::RoleMap<CounterPermission>
}

public fun create(
    ctx: &mut TxContext,
): (Capability, ID) {
    let counter_uid = object::new(ctx);
    let counter_id = object::uid_to_inner(&counter_uid);

    // Create a `CapabilityAdminPermissions` instance to configure the permissions
    // that will be needed by users to issue and revoke cabilities with the `RoleMap`.
    //
    // There are two actions that need to be configured with a permission of your choice:
    // * `add`: Permission required to add (issue) a new capability
    // * `revoke`: Permission required to revoke an existing capability
    //
    // In this example we use a specific permission for each action.
    //
    let capability_admin_permissions = role_map::new_capability_admin_permissions(
        counter::permission::add_capabilities(),
        counter::permission::revoke_capabilities(),
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
    // specific permissiions like `AddRoles`, `DeleteRoles` and `UpdateRoles` like we did
    // above to specifify the `CapabilityAdminPermissions`.
    //
   let role_admin_permissions = role_map::new_role_admin_permissions(
        counter::permission::manage_roles(),
        counter::permission::manage_roles(),
        counter::permission::manage_roles(),
    );

    let (roles, admin_cap) = role_map::new(
        counter_id,
        "super-admin",
        permission::super_admin_permissions(),
        role_admin_permissions,
        capability_admin_permissions,
        ctx,
    );

    let counter = Counter {
        id: counter_uid,
        0,
        roles,
    };
    transfer::share_object(counter);

    (admin_cap, counter_id)
}
```

Later on, the `Counter` can use the `RoleMap.is_capability_valid()` function to check
whether a provided capability has the required permission:

In the file `counter/counter.move`:
```Move
public fun increment(
    counter: &mut Counter,
    cap: &Capability,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(
        counter
            .roles
            .is_capability_valid(
                cap,
                &counter::permission::increment_counter(),
                clock,
                ctx,
            ),
        EPermissionDenied,
    );
    counter.value = counter.value + 1;
}
```

Using the shared Counter object would look like this:

In the file `tests/counter_tests.move`:
```Move
const INITIAL_TIME_FOR_TESTING: u64 = 1234;

/// Test capability lifecycle: creation, usage, revocation and destruction in a complete workflow.
///
/// This test validates:
/// - A capability can be created for the `counter-admin` role
/// - The Capability can be used to perform authorized actions
/// - The Capability can be revoked
/// - The Capability can be destroyed thereafter
#[test]

fun test_capability_lifecycle() {
    let super_admin_user = @0xAD;
    let counter_admin_user = @0xB0B;

    let mut scenario = ts::begin(super_admin_user);

    // Setup: Create Counter
    let (super_admin_cap, counter_id) = counter::create(ts::ctx(scenario));

    // Create an additional CounterAdmin role
    ts::next_tx(&mut scenario, super_admin_user);
    {
        let super_admin_cap = ts::take_from_sender<Capability>(scenario);
        let counter = ts::take_shared<Counter>(scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(scenario));

        // Initially only the super-admin cap should be tracked
        assert!(counter.roles().issued_capabilities().size() == 1, 0);

        counter
            .roles_mut()
            .create_role(
                &super_admin_cap,
                string::utf8(b"counter-admin"),
                permission::counter_admin_permissions(),
                &clock,
                ts::ctx(&mut scenario),
            );

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(scenario, super_admin_cap);
        ts::return_shared(counter);
    };

    // Issue the CounterAdmin capability to another user
    ts::next_tx(&mut scenario, super_admin_user);
    let counter_admin_cap_id = {
        let super_admin_cap = ts::take_from_sender<Capability>(scenario);
        let counter = ts::take_shared<Counter>(scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(scenario));

        let counter_cap = counter
            .roles_mut()
            .new_capability_without_restrictions(
                &super_admin_cap,
                &string::utf8(b"counter-admin"),
                &clock,
                ts::ctx(&mut scenario),
            );
        let counter_admin_cap_id = object::id(&counter_cap);
        transfer::public_transfer(counter_cap, counter_admin_user);

        // Verify all capabilities are tracked
        assert!(counter.roles().issued_capabilities().size() == 2, 1); // super-admin + counter
        assert!(counter.roles().issued_capabilities().contains(&counter_admin_cap_id), 2);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(scenario, super_admin_cap);
        ts::return_shared(counter);

        counter_admin_cap_id
    };

    // Use CounterAdmin capability to increment the counter
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(scenario);
        let counter = ts::take_shared<Counter>(scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(scenario));

        counter.increment(
            &counter_admin_cap,
            &clock,
            ts::ctx(&mut scenario),
        );

        assert!(counter.value() == 1, 3);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(scenario, counter_admin_cap);
        ts::return_shared(counter);
    };

    // Revoke the CounterAdmin capability
    ts::next_tx(&mut scenario, super_admin_user);
    {
        let super_admin_cap = ts::take_from_sender<Capability>(scenario);
        let counter = ts::take_shared<Counter>(scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(scenario));

        counter
            .roles_mut()
            .revoke_capability(
                &super_admin_cap,
                &counter_admin_cap_id,
                &clock,
                ts::ctx(&mut scenario),
            );

        // Verify capability was removed from the issued_capabilities list
        assert!(counter.roles().issued_capabilities().size() == 1, 4); // super-admin only
        assert!(!counter.roles().issued_capabilities().contains(&record_cap_id), 5);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(scenario, super_admin_cap);
        ts::return_shared(counter);
    };

    // The `counter_admin_user` can destroy the capability before it is revoked or after it is revoked.
    // Here we test destroying after revocation.
    // If the capability is destroyed before revocating it, the capability would be revoked automatically during `destroy_capability()`.
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(scenario);
        let counter = ts::take_shared<Counter>(scenario);

        counter
            .roles_mut()
            .destroy_capability(counter_admin_cap);

        ts::return_shared(counter);
    }
}
```