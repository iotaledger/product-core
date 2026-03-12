// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A role-based access control helper, mapping unique role identifiers to their associated permissions.
///
/// [!WARNING]
///
/// The current implementation of the `RoleMap` and `Capability` modules is a first iteration and is expected
/// to undergo significant changes in the future. The public interface is not yet stable, and we anticipate breaking changes
/// as we refine the design and implementation based on feedback and evolving requirements.
///
/// The final design and API of these modules will be released as part of the Audit Trail product, which will be
/// the first product to integrate these components.
///
/// A `RoleMap<P, D>` provides the following functionalities:
/// - Uses custom permission-types, defined by the integrating module, using the generic argument `P`
/// - Defines an initial role with a custom set of permissions (i.e. for an Admin role) and creates an initial
///   `Capability` for this role to allow later access control administration by the creator of the integrating module
/// - Allows to create, delete, and update roles and their permissions
/// - Allows to issue, revoke, and destroy `Capability`s associated with a specific role
/// - Validates `Capability`s against the defined roles to facilitate proper access control by the integrating module
///   (function `RoleMap.assert_capability_valid()`)
/// - All functions are access restricted by custom permissions defined during `RoleMap` instantiation
/// - Using the generic argument `D`, custom role-data can be stored as part of each role definition, to allow extended
///    access authorization by modules integrating the RoleMap
/// - Stores the initial admin role name in `initial_admin_role_name`
/// - Tracks active initial admin capability IDs in `initial_admin_cap_ids`
/// - Requires explicit initial-admin revoke/destroy APIs for those IDs
///
/// Examples:
/// - The TF product Audit Trails uses `RoleMap` to manage access to the audit trail records and their operations.
/// - The `TfComponents` package provides a "Hello World" like simple [`Counter` example](../examples/counter/README.md).
///
module tf_components::role_map;

use iota::clock::{Self, Clock};
use iota::event;
use iota::vec_map::{Self, VecMap};
use iota::vec_set::{Self, VecSet};
use iota::linked_table::{Self, LinkedTable};
use std::string::String;
use tf_components::capability::{Self, Capability};

/// Package version, incremented when the package is updated
///
/// The RoleMap itself is not a shared object and therefore would not need to compare the package version of a
/// stored object with the package-version of a called function (as been described
/// [here](https://docs.iota.org/developer/iota-101/move-overview/package-upgrades/upgrade#managing-versioned-shared-objects)).
/// 
/// However, since the RoleMap is designed to be directly called via references provided by the integrating modules,
/// we need to do these version checks also in functions of the `role_map` module. Otherwise we would need to implement
/// wrapper functions for all called `role_map` functions in the integrating modules just to do the version checks, which would
/// cause a lot of redundancy for code and docs, would be hard to maintain and error-prone.
/// 
/// The PACKAGE_VERSION value will differ from the package version of the integrating module but the `package_version` value
/// serialized together with the RoleMap will identify the TfComponents package version used as a dependency when the integrating
/// module has been published.
/// 
/// We need to distinguish three edge case situations here:
/// * In case a function of the integrating module is called with an instance of the integrating shared object, that has the correct
///   package version (function package version == object package version), the version check in the role_map module will also be ok.
/// * In case an upgraded integrating package has been linked against an untempered TfComponents package (same version as been used
///   when the integrating shared object has been stored) and a function of the integrating module is called with an outdated instance
///   of the integrating shared object (function package version > object package version), the version check in the role_map module
///   will still be ok.
///   As this only happens during calls to functions of the `role_map` module, there is no risk of incompatibilities, meaning that there
///   is no need to abort the call as Role & Capability management functionality has not been changed.
/// * In case an upgraded integrating package has been linked against an upgraded TfComponents package and a function of the integrating
///   module is called with an outdated instance of the integrating shared object (function package version > object package version),
///   the version check in the role_map module will also fail and the aborting function will force users to migrate the shared object
///   which is equivalent to failing version checks in the integrating module.
const PACKAGE_VERSION: u64 = 1;

// =============== Errors ======================

#[error]
const ERoleDoesNotExist: vector<u8> =
    b"The specified role, directly specified or specified by a capability, does not exist in the `RoleMap` mapping";
#[error]
const ECapabilityHasBeenRevoked: vector<u8> =
    b"The provided capability has been revoked and is no longer valid";
#[error]
const ECapabilityTargetKeyMismatch: vector<u8> =
    b"The target_key associated with the provided capability does not match the target_key of the `RoleMap`";
#[error]
const ECapabilityTimeConstraintsNotMet: vector<u8> =
    b"The capability's time constraints are not currently met either due to `valid_from` or `valid_until` restrictions";
#[error]
const ECapabilityIssuedToMismatch: vector<u8> =
    b"The capability is restricted to a specific address which does not match the caller's address";
#[error]
const ECapabilityPermissionDenied: vector<u8> =
    b"The role associated with provided capability does not have the required permission";
#[error]
const ECapabilityToRevokeHasAlreadyBeenRevoked: vector<u8> =
    b"The capability that shall be revoked has already been revoked";
#[error]
const EInitialAdminPermissionsInconsistent: vector<u8> =
    b"The initial admin role must include all configured role and capability admin permissions";
#[error]
const EInitialAdminRoleCannotBeDeleted: vector<u8> = b"The initial admin role cannot be deleted";
#[error]
const EInitialAdminCapabilityMustBeExplicitlyDestroyed: vector<u8> =
    b"Initial admin capabilities cannot be revoked or destroyed via this function. Use revoke_initial_admin_capability or destroy_initial_admin_capability instead";
#[error]
const ECapabilityIsNotInitialAdmin: vector<u8> =
    b"This capability is not an initial admin capability";
#[error]
const EPackageVersionMismatch: vector<u8> =
    b"The package version of the RoleMap instance does not match the current package version";
#[error]
const EMigrationUnexpectedPackageVersion: vector<u8> =
    b"To migrated RoleMap instances, the package version of the RoleMap instance needs to lower the current package version, which is not the case";

// =============== Events ====================

/// Emitted when a capability is issued
public struct CapabilityIssued has copy, drop {
    target_key: ID,
    capability_id: ID,
    role: String,
    issued_to: Option<address>,
    valid_from: Option<u64>,
    valid_until: Option<u64>,
}

/// Emitted when a capability is destroyed
public struct CapabilityDestroyed has copy, drop {
    target_key: ID,
    capability_id: ID,
    role: String,
    issued_to: Option<address>,
    valid_from: Option<u64>,
    valid_until: Option<u64>,
}

/// Emitted when a capability is revoked
public struct CapabilityRevoked has copy, drop {
    target_key: ID,
    capability_id: ID,
    valid_until: u64,
}

/// Emitted when a role is created
public struct RoleCreated<P: copy + drop, D: copy + drop> has copy, drop {
    target_key: ID,
    role: String,
    permissions: VecSet<P>,
    data: Option<D>,
    created_by: address,
    timestamp: u64,
}

/// Emitted when a role is deleted
public struct RoleDeleted has copy, drop {
    target_key: ID,
    role: String,
    deleted_by: address,
    timestamp: u64,
}

/// Emitted when a role's is updated
public struct RoleUpdated<P: copy + drop, D: copy + drop> has copy, drop {
    target_key: ID,
    role: String,
    new_permissions: VecSet<P>,
    new_data: Option<D>,
    updated_by: address,
    timestamp: u64,
}

// =============== Core Types ====================

/// Defines the permissions required to administer roles in this RoleMap
public struct RoleAdminPermissions<P: copy + drop> has copy, drop, store {
    /// Permission required to add a new role
    add: P,
    /// Permission required to delete an existing role
    delete: P,
    /// Permission required to update permissions associated with an existing role
    update: P,
}

/// Defines the permissions required to administer capabilities in this RoleMap
public struct CapabilityAdminPermissions<P: copy + drop> has copy, drop, store {
    /// Permission required to add (issue) a new capability
    add: P,
    /// Permission required to revoke an existing capability
    revoke: P,
}

/// The RoleMap structure mapping role names to their associated permissions and role-data
///
/// Generic parameters:
/// * P defines the permission type used by the integrating module
///   (i.e. audit_trail::Permission)
/// * D defines the role-data type. Each role has role-data which can be used by integrating modules to provide
///   explanations or to perform additional access control constraints, performed by additional access control checks.
///   To perform additional access control checks, integrating modules need to wrap the `RoleMap::is_capability_valid()` call
///   in their own `is_capability_valid()` implementation, use this wrapper function for evaluating the additional checks
///   and use the role-data to store role specific variables. `RoleMap::is_capability_valid()` itself will ignore the role-data.
public struct RoleMap<P: copy + drop, D: copy + drop> has store {
    /// Identifies the scope (or domain) managed by the RoleMap.  Usually this is the ID of the managed onchain object
    /// (i.e. an audit trail). You can also derive an arbitrary ID value reused by several managed onchain objects
    /// to share the used roles and capabilities between these objects.
    target_key: ID,
    /// Mapping of role names to their associated permissions
    roles: VecMap<String, Role<P, D>>,
    /// Name of the initial admin role created by `new`.
    /// The RoleMap uses this to protect that role from unsafe changes.
    initial_admin_role_name: String,
    /// Denylist of all revoked capability IDs mapped to their optional valid_until timestamp (if any).
    /// If a revoked capability has no valid_until timestamp, its u64 value is set to 0.
    /// The optional valid_until timestamp allows for automatic removal of expired capabilities to keep the list as
    /// short as possible (see function `cleanup_revoked_capabilities_list()` for more details).
    revoked_capabilities: LinkedTable<ID, u64>,
    /// IDs of active capabilities for the initial admin role.
    /// These IDs cannot be removed through generic revoke/destroy functions.
    /// Use `revoke_initial_admin_capability` or `destroy_initial_admin_capability` instead.
    initial_admin_cap_ids: VecSet<ID>,
    /// Permissions required to administer roles in this RoleMap
    role_admin_permissions: RoleAdminPermissions<P>,
    /// Permissions required to administer capabilities in this RoleMap
    capability_admin_permissions: CapabilityAdminPermissions<P>,
    /// Package version - See `PACKAGE_VERSION` above for more details
    version: u64,    
}

// Definition of role specific access permissions and role-data
// See `RoleMap<P, D>` above for more details
public struct Role<P: copy + drop, D: copy + drop> has copy, drop, store {
    permissions: VecSet<P>,
    data: Option<D>,
}

// ========== Role & Capability AdminPermissions Functions ===========

public fun new_role_admin_permissions<P: copy + drop>(
    add: P,
    delete: P,
    update: P,
): RoleAdminPermissions<P> {
    RoleAdminPermissions {
        add,
        delete,
        update,
    }
}

/// Returns the `add` permission of the `RoleAdminPermissions`
public fun role_admin_permissions_add<P: copy + drop>(self: &RoleAdminPermissions<P>): &P {
    &self.add
}

/// Returns the `delete` permission of the `RoleAdminPermissions`
public fun role_admin_permissions_delete<P: copy + drop>(self: &RoleAdminPermissions<P>): &P {
    &self.delete
}

/// Returns the `update` permission of the `RoleAdminPermissions`
public fun role_admin_permissions_update<P: copy + drop>(self: &RoleAdminPermissions<P>): &P {
    &self.update
}

public fun new_capability_admin_permissions<P: copy + drop>(
    add: P,
    revoke: P,
): CapabilityAdminPermissions<P> {
    CapabilityAdminPermissions {
        add,
        revoke,
    }
}

/// Returns the `add` permission of the `CapabilityAdminPermissions`
public fun capability_admin_permissions_add<P: copy + drop>(self: &CapabilityAdminPermissions<P>): &P {
    &self.add
}

/// Returns the `revoke` permission of the `CapabilityAdminPermissions`
public fun capability_admin_permissions_revoke<P: copy + drop>(self: &CapabilityAdminPermissions<P>): &P {
    &self.revoke
}

// ============ RoleMap Functions ====================

/// Create a new RoleMap with an initial admin role
/// The initial admin role is created with the specified name and permissions
/// An initial admin capability is created and returned alongside the RoleMap
/// The initial admin capability has no restrictions (no address, valid_from, or valid_until)
/// The target_key is associated with both the RoleMap and the initial admin capability
/// Returns the newly created RoleMap and the initial admin capability
///
/// Parameters
/// ----------
/// - target_key:
///   The target_key to associate this RoleMap with the initial admin capability
///   and all other created capabilities. Usually this is the ID of the managed onchain object
///   (i.e. an audit_trail::AuditTrail or the tf_components::Counter).
/// - `initial_admin_role_name`: The name of the initial admin role
/// - `initial_admin_role_permissions`: Permissions granted to that role.
/// - `role_admin_permissions`: Permissions required to manage roles.
/// - `capability_admin_permissions`: Permissions required to manage
///    capabilities.
/// - `ctx`: The transaction context
///
/// Errors:
/// - Aborts with `EInitialAdminPermissionsInconsistent` if `initial_admin_role_permissions`
///   does not include all permissions configured in `role_admin_permissions` and
///   `capability_admin_permissions`.
public fun new<P: copy + drop, D: copy + drop>(
    target_key: ID,
    initial_admin_role_name: String,
    initial_admin_role_permissions: VecSet<P>,
    role_admin_permissions: RoleAdminPermissions<P>,
    capability_admin_permissions: CapabilityAdminPermissions<P>,
    ctx: &mut TxContext,
): (RoleMap<P, D>, Capability) {
    assert!(
        has_required_admin_permissions(
            &initial_admin_role_permissions,
            &role_admin_permissions,
            &capability_admin_permissions,
        ),
        EInitialAdminPermissionsInconsistent,
    );

    let mut roles = vec_map::empty<String, Role<P, D>>();
    roles.insert(
        copy initial_admin_role_name,
        new_role(initial_admin_role_permissions, std::option::none()),
    );

    let admin_cap = capability::new_capability(
        copy initial_admin_role_name,
        target_key,
        option::none(),
        option::none(),
        option::none(),
        ctx,
    );
    let mut initial_admin_cap_ids = vec_set::empty<ID>();
    initial_admin_cap_ids.insert(admin_cap.id());
    let role_map = RoleMap {
        roles,
        initial_admin_role_name,
        role_admin_permissions,
        capability_admin_permissions,
        target_key,
        revoked_capabilities: linked_table::new<ID, u64>(ctx),
        initial_admin_cap_ids,
        version: PACKAGE_VERSION,
    };

    (role_map, admin_cap)
}

/// Safely destroys a RoleMap.
/// Will destroy all stored roles and capabilities.
public fun destroy<P: copy + drop, D: copy + drop>(self: RoleMap<P, D>) {
    let RoleMap {
        roles: _,
        initial_admin_role_name: _,
        role_admin_permissions: _,
        capability_admin_permissions: _,
        target_key: _,
        mut revoked_capabilities,
        initial_admin_cap_ids: _,
        version: _,
    } = self;

    while (!revoked_capabilities.is_empty()) {
       revoked_capabilities.pop_front();
    };
    revoked_capabilities.destroy_empty();
}

/// Migrate a RoleMap to the latest package version
///
/// This function needs to be called by the integrating modules `migrate` function.
public fun migrate<P: copy + drop, D: copy + drop>(
    mut self: RoleMap<P, D>,
    _clock: &Clock,
    _ctx: &TxContext,
): RoleMap<P, D> {
    assert!(self.version < PACKAGE_VERSION, EMigrationUnexpectedPackageVersion);
    self.version = PACKAGE_VERSION;
    self
}

// ============ Role Functions ====================

/// Get the permissions associated with a specific role.
/// Aborts with `ERoleDoesNotExist` if the role does not exist.
public fun get_role_permissions<P: copy + drop, D: copy + drop>(
    self: &RoleMap<P, D>,
    role: &String,
): &VecSet<P> {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    assert!(vec_map::contains(&self.roles, role), ERoleDoesNotExist);
    &vec_map::get(&self.roles, role).permissions
}

/// Get the role-data associated with a specific role.
/// Aborts with `ERoleDoesNotExist` if the role does not exist.
public fun get_role_data<P: copy + drop, D: copy + drop>(
    self: &RoleMap<P, D>,
    role: &String,
): &Option<D> {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    assert!(vec_map::contains(&self.roles, role), ERoleDoesNotExist);
    &vec_map::get(&self.roles, role).data
}

/// Create a new role consisting of a role name and associated permissions
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - The provided capability needs to grant the `RoleAdminPermissions::add` permission.
///
/// Sends a `RoleCreated` event upon successful update.
public fun create_role<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    role: String,
    permissions: VecSet<P>,
    data: Option<D>,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.role_admin_permissions.add,
        clock,
        ctx,
    );

    vec_map::insert(&mut self.roles, role, new_role(permissions, data));

    event::emit(RoleCreated {
        target_key: self.target_key,
        role,
        permissions,
        data,
        created_by: ctx.sender(),
        timestamp: clock::timestamp_ms(clock),
    });
}

/// Delete an existing role
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - The provided capability needs to grant the `RoleAdminPermissions::delete` permission.
/// - Aborts with `ERoleDoesNotExist` if the specified role does not exist in the role_map.
///
/// Sends a `RoleDeleted` event upon successful update.
public fun delete_role<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    role: &String,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.role_admin_permissions.delete,
        clock,
        ctx,
    );

    assert!(vec_map::contains(&self.roles, role), ERoleDoesNotExist);
    assert!(*role != self.initial_admin_role_name, EInitialAdminRoleCannotBeDeleted);
    vec_map::remove(&mut self.roles, role);

    event::emit(RoleDeleted {
        target_key: self.target_key,
        role: *role,
        deleted_by: ctx.sender(),
        timestamp: clock::timestamp_ms(clock),
    });
}

/// Update permissions and role_data associated with an existing role
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - The provided capability needs to grant the `RoleAdminPermissions::update` permission.
/// - Aborts with `ERoleDoesNotExist` if the specified role does not exist in the role_map.
/// - Aborts with `EInitialAdminPermissionsInconsistent` if `new_permissions`
///   does not include all permissions configured in `role_admin_permissions` and
///   `capability_admin_permissions`.
///
/// Sends a `RoleUpdated` event upon successful update.
public fun update_role<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    role_name: &String,
    new_permissions: VecSet<P>,
    data: Option<D>,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.role_admin_permissions.update,
        clock,
        ctx,
    );

    if (*role_name == self.initial_admin_role_name) {
        assert!(
            has_required_admin_permissions(
                &new_permissions,
                &self.role_admin_permissions,
                &self.capability_admin_permissions,
            ),
            EInitialAdminPermissionsInconsistent,
        );
    };

    assert!(vec_map::contains(&self.roles, role_name), ERoleDoesNotExist);
    let role = vec_map::get_mut(&mut self.roles, role_name);

    role.permissions = new_permissions;
    role.data = data;

    event::emit(RoleUpdated {
        target_key: self.target_key,
        role: *role_name,
        new_permissions,
        new_data: data,
        updated_by: ctx.sender(),
        timestamp: clock::timestamp_ms(clock),
    });
}

/// Indicates if the specified role exists in the role_map
public fun has_role<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>, role: &String): bool {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    vec_map::contains(&self.roles, role)
}

/// Returns the permissions associated with a `Role`
public fun role_permissions<P: copy + drop, D: copy + drop>(self: &Role<P, D>): &VecSet<P> {
    &self.permissions
}

/// Returns the data associated with a `Role`
public fun role_data<P: copy + drop, D: copy + drop>(self: &Role<P, D>): &Option<D> {
    &self.data
}

public(package) fun new_role<P: copy + drop, D: copy + drop>(
    permissions: VecSet<P>,
    data: Option<D>,
): Role<P, D> {
    Role {
        permissions,
        data,
    }
}

/// ===== Capability Functions =======

/// Indicates if a provided capability is valid.
///
/// A capability is considered valid if:
/// - The capability's target_key matches the RoleMap's target_key.
///   Aborts with ECapabilityTargetKeyMismatch if not matching.
/// - The role value specified by the capability exists in the `RoleMap` mapping.
///   Aborts with `ERoleDoesNotExist` if the role does not exist.
/// - The role associated with the capability contains the permission specified by the `permission` argument.
///   Aborts with `ECapabilityPermissionDenied` if the permission is not granted by the role.
/// - The capability has not been revoked (is included in the `issued_capabilities` set).
///   Aborts with `ECapabilityHasBeenRevoked` if revoked.
/// - The capability is currently active, based on its time restrictions (if any).
///   Aborts with `ECapabilityTimeConstraintsNotMet`, if the current time is outside the `valid_from` and `valid_until` range.
/// - If the capability is restricted to a specific address, the caller's address matches the sender of the transaction.
///   Aborts with `ECapabilityIssuedToMismatch` if the addresses do not match.
///
/// Parameters
/// ----------
/// - cap: Reference to the capability to be validated.
/// - permission: The permission to check against the capability's role.
/// - clock: Reference to a Clock instance for time-based validation.
/// - ctx: Reference to the transaction context for accessing the caller's address.
///
/// Aborts if the capability is invalid for this RoleMap and permission.
public fun assert_capability_valid<P: copy + drop, D: copy + drop>(
    self: &RoleMap<P, D>,
    cap: &Capability,
    permission: &P,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    assert!(self.target_key == cap.target_key(), ECapabilityTargetKeyMismatch);

    let permissions = self.get_role_permissions(cap.role());
    assert!(vec_set::contains(permissions, permission), ECapabilityPermissionDenied);

    assert!(!self.revoked_capabilities.contains(cap.id()), ECapabilityHasBeenRevoked);

    if (cap.valid_from().is_some() || cap.valid_until().is_some()) {
        assert!(cap.is_currently_valid(clock), ECapabilityTimeConstraintsNotMet);
    };

    if (cap.issued_to().is_some()) {
        let caller = ctx.sender();
        let issued_to_addr = cap.issued_to().borrow();
        assert!(*issued_to_addr == caller, ECapabilityIssuedToMismatch);
    };
}

/// Create a new capability
///
/// Parameters
/// ----------
/// - cap: Reference to the capability used to authorize the creation of the new capability.
///   Needs to grant the `CapabilityAdminPermissions::add` permission.
/// - role: The role to be assigned to the new capability.
/// - issued_to: Optional address restriction for the new capability.
/// - valid_from: Optional start time (in milliseconds since Unix epoch) for the new capability.
/// - valid_until: Optional. Last point in time where the capability is valid (in milliseconds since Unix epoch).
/// - clock: Reference to a Clock instance for time-based validation.
/// - ctx: Reference to the transaction context.
///
/// Returns the newly created capability.
///
/// Errors:
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - Aborts with `ERoleDoesNotExist` if the specified role does not exist in the role_map.
/// - Aborts with `tf_components::capability::EValidityPeriodInconsistent` if the provided valid_from and valid_until are inconsistent.
///
/// Sends a `CapabilityIssued` event upon successful creation.
public fun new_capability<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    role: &String,
    issued_to: Option<address>,
    valid_from: Option<u64>,
    valid_until: Option<u64>,
    clock: &Clock,
    ctx: &mut TxContext,
): Capability {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.capability_admin_permissions.add,
        clock,
        ctx,
    );

    assert!(self.roles.contains(role), ERoleDoesNotExist);
    let new_cap = capability::new_capability(
        *role,
        self.target_key,
        issued_to,
        valid_from,
        valid_until,
        ctx,
    );
    self.issue_capability(&new_cap);
    new_cap
}

/// Destroy an existing capability
/// Every owner of a capability is allowed to destroy it when no longer needed.
/// This operation is intentionally not gated by `CapabilityAdminPermissions::revoke`.
///
/// Initial admin capabilities cannot be destroyed via this function.
/// Use `destroy_initial_admin_capability` instead.
///
/// Sends a `CapabilityDestroyed` event upon successful destruction.
public fun destroy_capability<P: copy + drop, D: copy + drop>(self: &mut RoleMap<P, D>, cap_to_destroy: Capability) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    assert!(self.target_key == cap_to_destroy.target_key(), ECapabilityTargetKeyMismatch);
    assert!(
        !self.initial_admin_cap_ids.contains(&cap_to_destroy.id()),
        EInitialAdminCapabilityMustBeExplicitlyDestroyed,
    );

    if (self.revoked_capabilities.contains(cap_to_destroy.id())) {
        self.revoked_capabilities.remove(cap_to_destroy.id());
    };

    event::emit(CapabilityDestroyed {
        target_key: self.target_key,
        capability_id: cap_to_destroy.id(),
        role: *cap_to_destroy.role(),
        issued_to: *cap_to_destroy.issued_to(),
        valid_from: *cap_to_destroy.valid_from(),
        valid_until: *cap_to_destroy.valid_until(),
     });

    cap_to_destroy.destroy();
}

/// Revoke an existing capability
///
/// Initial admin capabilities cannot be revoked via this function.
/// Use `revoke_initial_admin_capability` instead.
///
/// Sends a `CapabilityRevoked` event upon successful revocation.
/// 
/// Parameters
/// ----------
/// - cap: Reference to the capability used to authorize the revocation of the `cap_to_revoke` capability.
///   Needs to grant the `CapabilityAdminPermissions::revoke` permission.
/// - cap_to_revoke: 
///   The capability to be revoked is specified by its ID.
///   The user of this function is responsible to only pass `cap_to_revoke` values that meet the following constraints:
///   * A capability with the provided `cap_to_revoke` ID exists
///   * The capability specified by `cap_to_revoke` has been issued by this RoleMap instance
///   * The optional `valid_until` value of the capability specified by `cap_to_revoke` has not expired
///     (in this case there would be no need to revoke it)
///   These checks should be performed by the user of this function to prevent the internally managed `revoked_capabilities`
///   list from being swamped with unnecessary capability ids. Although there is no maximum size to be taken into account
///   the list should be held as small as possible. The function itself will not evaluate any of the above listed checks.
/// - cap_to_revoke_valid_until: If specified, the `valid_until` value of the `cap_to_revoke`.
///   This value will be stored in the internally managed `revoked_capabilities` list and can be used later on to 
///   do automatic list cleanups by removing already expired capabilities from the list.
/// - clock: Reference to a Clock instance for time-based validation.
/// - ctx: Reference to the transaction context.
/// 
/// Errors:
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - Aborts with `ECapabilityToRevokeHasAlreadyBeenRevoked` if `cap_to_revoke` has already been revoked.
/// - Aborts with `EInitialAdminCapabilityMustBeExplicitlyDestroyed` if `cap_to_revoke` is an initial admin capability.
public fun revoke_capability<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    cap_to_revoke: ID,
    cap_to_revoke_valid_until: Option<u64>,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.capability_admin_permissions.revoke,
        clock,
        ctx,
    );

    assert!(
        !self.initial_admin_cap_ids.contains(&cap_to_revoke),
        EInitialAdminCapabilityMustBeExplicitlyDestroyed,
    );

    self.add_cap_to_revoke_list(cap_to_revoke, cap_to_revoke_valid_until)
}

/// Remove expired entries from the `revoked_capabilities` denylist.
///
/// Iterates through the revoked capabilities list and removes every entry whose
/// `valid_until` timestamp is **non-zero** and **less than** the current clock time,
/// because those capabilities are already naturally expired and no longer need to
/// occupy space in the denylist.
///
/// Entries with `valid_until == 0` (i.e. capabilities that had no expiry) are kept,
/// since they remain potentially valid and must stay on the denylist.
///
/// Parameters
/// ----------
/// - cap: Reference to the capability used to authorize this operation.
///   Needs to grant the `CapabilityAdminPermissions::revoke` permission.
/// - clock: Reference to a Clock instance for obtaining the current timestamp.
/// - ctx: Reference to the transaction context.
///
/// Errors:
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
public fun cleanup_revoked_capabilities_list<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.capability_admin_permissions.revoke,
        clock,
        ctx,
    );

    let now = clock::timestamp_ms(clock);
    let mut current_key = *self.revoked_capabilities.front();

    while (current_key.is_some()) {
        let key = *current_key.borrow();
        let valid_until = *self.revoked_capabilities.borrow(key);
        // Peek at the next key before potentially removing the current node.
        let next_key = *self.revoked_capabilities.next(key);

        if (valid_until > 0 && valid_until < now) {
            self.revoked_capabilities.remove(key);
        };

        current_key = next_key;
    };
}

/// Destroy an initial admin capability.
///
/// This is the only way to destroy a capability associated with the initial admin role.
/// Every owner of an initial admin capability is allowed to destroy it when no longer needed.
/// This operation is intentionally not gated by `CapabilityAdminPermissions::revoke`.
///
/// WARNING: If all initial admin capabilities are destroyed, the RoleMap will be permanently
/// sealed with no admin access possible.
///
/// Sends a `CapabilityDestroyed` event upon successful destruction.
///
/// Errors:
/// - Aborts with `ECapabilityTargetKeyMismatch` if the capability's target_key does not match.
/// - Aborts with `ECapabilityIsNotInitialAdmin` if the capability is not an initial admin capability.
public fun destroy_initial_admin_capability<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap_to_destroy: Capability,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    assert!(self.target_key == cap_to_destroy.target_key(), ECapabilityTargetKeyMismatch);
    assert!(
        self.initial_admin_cap_ids.contains(&cap_to_destroy.id()),
        ECapabilityIsNotInitialAdmin,
    );

    if (self.revoked_capabilities.contains(cap_to_destroy.id())) {
            self.revoked_capabilities.remove(cap_to_destroy.id());
    };
    self.initial_admin_cap_ids.remove(&cap_to_destroy.id());

    event::emit(CapabilityDestroyed {
        target_key: self.target_key,
        capability_id: cap_to_destroy.id(),
        role: *cap_to_destroy.role(),
        issued_to: *cap_to_destroy.issued_to(),
        valid_from: *cap_to_destroy.valid_from(),
        valid_until: *cap_to_destroy.valid_until(),
     });

    cap_to_destroy.destroy();
}

/// Revoke an initial admin capability.
///
/// This is the only way to revoke a capability associated with the initial admin role.
/// Requires `CapabilityAdminPermissions::revoke` permission.
///
/// WARNING: If all initial admin capabilities are revoked, the RoleMap will be permanently
/// sealed with no admin access possible.
///
/// Sends a `CapabilityRevoked` event upon successful revocation.
/// 
/// See function `revoke_capability()` for parameter documentation. 
///
/// Errors:
/// - Aborts with any error documented by `assert_capability_valid` if the provided capability fails authorization checks.
/// - The provided capability needs to grant the `CapabilityAdminPermissions::revoke` permission.
/// - Aborts with `ECapabilityToRevokeHasAlreadyBeenRevoked` if `cap_to_revoke` has already been revoked.
/// - Aborts with `ECapabilityIsNotInitialAdmin` if `cap_to_revoke` is not an initial admin capability.
public fun revoke_initial_admin_capability<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap: &Capability,
    cap_to_revoke: ID,
    cap_to_revoke_valid_until: Option<u64>,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(self.version == PACKAGE_VERSION, EPackageVersionMismatch);
    self.assert_capability_valid(
        cap,
        &self.capability_admin_permissions.revoke,
        clock,
        ctx,
    );

    assert!(self.initial_admin_cap_ids.contains(&cap_to_revoke), ECapabilityIsNotInitialAdmin);

    self.initial_admin_cap_ids.remove(&cap_to_revoke);
    self.add_cap_to_revoke_list(cap_to_revoke, cap_to_revoke_valid_until)
}

/// Checks if the provided permissions include all required admin permissions
///
/// Returns true if the provided permissions include all required admin
fun has_required_admin_permissions<P: copy + drop>(
    permissions: &VecSet<P>,
    role_admin_permissions: &RoleAdminPermissions<P>,
    capability_admin_permissions: &CapabilityAdminPermissions<P>,
): bool {
    permissions.contains(&role_admin_permissions.add) &&
        permissions.contains(&role_admin_permissions.delete) &&
        permissions.contains(&role_admin_permissions.update) &&
        permissions.contains(&capability_admin_permissions.add) &&
        permissions.contains(&capability_admin_permissions.revoke)
}

/// Issues a new capability
fun issue_capability<P: copy + drop, D: copy + drop>(self: &mut RoleMap<P, D>, new_cap: &Capability) {
    if (new_cap.role() == &self.initial_admin_role_name) {
        self.initial_admin_cap_ids.insert(new_cap.id());
    };

    event::emit(CapabilityIssued {
        target_key: self.target_key,
        capability_id: new_cap.id(),
        role: *new_cap.role(),
        issued_to: *new_cap.issued_to(),
        valid_from: *new_cap.valid_from(),
        valid_until: *new_cap.valid_until(),
    });
}

/// Add a capability to the revoke list
fun add_cap_to_revoke_list<P: copy + drop, D: copy + drop>(
    self: &mut RoleMap<P, D>,
    cap_to_revoke: ID,
    cap_to_revoke_valid_until: Option<u64>
) {
    assert!(!self.revoked_capabilities.contains(cap_to_revoke), ECapabilityToRevokeHasAlreadyBeenRevoked);

    let valid_until = cap_to_revoke_valid_until.borrow_with_default(&0);
    self.revoked_capabilities.push_back(cap_to_revoke, *valid_until);

    event::emit(CapabilityRevoked {
        target_key: self.target_key,
        capability_id: cap_to_revoke,
        valid_until: *valid_until,
    });
}

// =============== Getter Functions ======================

/// Returns the size of the role_map, the number of managed roles
public fun size<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): u64 {
    vec_map::size(&self.roles)
}

/// Returns the target_key associated with the role_map
public fun target_key<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): ID {
    self.target_key
}

/// Returns the role admin permissions associated with the role_map
public fun role_admin_permissions<P: copy + drop, D: copy + drop>(
    self: &RoleMap<P, D>,
): &RoleAdminPermissions<P> {
    &self.role_admin_permissions
}

/// Returns the capability admin permissions associated with the role_map
public fun capability_admin_permissions<P: copy + drop, D: copy + drop>(
    self: &RoleMap<P, D>,
): &CapabilityAdminPermissions<P> {
    &self.capability_admin_permissions
}

/// Returns the list of revoked capabilities
public fun revoked_capabilities<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): &LinkedTable<ID,u64> {
    &self.revoked_capabilities
}

/// Returns the initial admin role name
public fun initial_admin_role_name<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): &String {
    &self.initial_admin_role_name
}

/// Returns the IDs of active initial admin capabilities
public fun initial_admin_cap_ids<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): &VecSet<ID> {
    &self.initial_admin_cap_ids
}

/// Returns all roles managed by the role_map
public fun roles<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): &VecMap<String, Role<P, D>> {
    &self.roles
}

/// Returns the package version of the RoleMap instance
public fun version<P: copy + drop, D: copy + drop>(self: &RoleMap<P, D>): u64 {
    self.version
}

