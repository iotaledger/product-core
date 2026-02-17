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
/// A `RoleMap<P>` provides the following functionalities:
/// - Uses custom permission-types, defined by the integrating module, using the generic argument `P`
/// - Defines an initial role with a custom set of permissions (i.e. for an Admin role) and creates an initial
///   `Capability` for this role to allow later access control administration by the creator of the integrating module
/// - Allows to create, delete, and update roles and their permissions
/// - Allows to issue, revoke, and destroy `Capability`s associated with a specific role
/// - Validates `Capability`s against the defined roles to facilitate proper access control by the integrating module
///   (function `RoleMap.is_capability_valid()`)
/// - All functions are access restricted by custom permissions defined during `RoleMap` instantiation
///
/// Examples:
/// - The TF product Audit Trails uses `RoleMap` to manage access to the audit trail records and their operations.
/// - The `TfComponents` package provides a "Hello World" like simple [`Counter` example](../examples/counter/README.md).
///
module tf_components::role_map;

use iota::{clock::Clock, event, vec_map::{Self, VecMap}, vec_set::{Self, VecSet}};
use std::string::String;
use tf_components::capability::{Self, Capability};

// =============== Errors ======================

#[error]
const ERoleDoesNotExist: vector<u8> =
    b"The specified role, directly specified or specified by a capability, does not exist in the `RoleMap` mapping";
#[error]
const ECapabilityHasBeenRevoked: vector<u8> =
    b"The provided capability has been revoked and is no longer valid";
#[error]
const ECapabilitySecurityVaultIdMismatch: vector<u8> =
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
const ECapabilityNotIssued: vector<u8> =
    b"The specified capability is not currently issued by this `RoleMap`";
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

/// Emitted when a capability is revoked or destroyed
public struct CapabilityRevoked has copy, drop {
    target_key: ID,
    capability_id: ID,
}

/// Emitted when a role is created
public struct RoleCreated has copy, drop {
    target_key: ID,
    role: String,
}

/// Emitted when a role is removed
public struct RoleRemoved has copy, drop {
    target_key: ID,
    role: String,
}

/// Emitted when a role is updated
public struct RoleUpdated has copy, drop {
    target_key: ID,
    role: String,
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

/// The RoleMap structure mapping role names to their associated permissions
/// Generic parameter P defines the permission type used by the integrating module
/// (i.e. tf_components::CounterPermission or audit_trail::Permission)
public struct RoleMap<P: copy + drop> has copy, drop, store {
    /// Identifies the scope (or domain) managed by the RoleMap.  Usually this is the ID of the managed onchain object
    /// (i.e. an audit trail). You can also derive an arbitrary ID value reused by several managed onchain objects
    /// to share the used roles and capabilities between these objects.
    target_key: ID,
    /// Mapping of role names to their associated permissions
    roles: VecMap<String, VecSet<P>>,
    /// Name of the initial admin role created by `new`.
    initial_admin_role_name: String,
    /// Allowlist of all issued capability IDs
    issued_capabilities: VecSet<ID>,
    /// Capability IDs currently issued for the initial admin role.
    initial_admin_cap_ids: VecSet<ID>,
    /// Permissions required to administer roles in this RoleMap
    role_admin_permissions: RoleAdminPermissions<P>,
    /// Permissions required to administer capabilities in this RoleMap
    capability_admin_permissions: CapabilityAdminPermissions<P>,
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

public fun new_capability_admin_permissions<P: copy + drop>(
    add: P,
    revoke: P,
): CapabilityAdminPermissions<P> {
    CapabilityAdminPermissions {
        add,
        revoke,
    }
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
public fun new<P: copy + drop>(
    target_key: ID,
    initial_admin_role_name: String,
    initial_admin_role_permissions: VecSet<P>,
    role_admin_permissions: RoleAdminPermissions<P>,
    capability_admin_permissions: CapabilityAdminPermissions<P>,
    ctx: &mut TxContext,
): (RoleMap<P>, Capability) {
    assert!(
        has_required_admin_permissions(
            &initial_admin_role_permissions,
            &role_admin_permissions,
            &capability_admin_permissions,
        ),
        EInitialAdminPermissionsInconsistent,
    );

    let mut roles = vec_map::empty<String, VecSet<P>>();
    roles.insert(copy initial_admin_role_name, initial_admin_role_permissions);

    let admin_cap = capability::new_capability(
        copy initial_admin_role_name,
        target_key,
        option::none(),
        option::none(),
        option::none(),
        ctx,
    );
    let mut issued_capabilities = vec_set::empty<ID>();
    issued_capabilities.insert(admin_cap.id());
    let mut initial_admin_cap_ids = vec_set::empty<ID>();
    initial_admin_cap_ids.insert(admin_cap.id());
    let role_map = RoleMap {
        roles,
        initial_admin_role_name,
        role_admin_permissions,
        capability_admin_permissions,
        target_key,
        issued_capabilities,
        initial_admin_cap_ids,
    };

    (role_map, admin_cap)
}

/// Get the permissions associated with a specific role.
/// Aborts with ERoleDoesNotExist if the role does not exist.
public fun get_role_permissions<P: copy + drop>(self: &RoleMap<P>, role: &String): &VecSet<P> {
    assert!(vec_map::contains(&self.roles, role), ERoleDoesNotExist);
    vec_map::get(&self.roles, role)
}

/// Create a new role consisting of a role name and associated permissions
public fun create_role<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    role: String,
    permissions: VecSet<P>,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.is_capability_valid(
        cap,
        &self.role_admin_permissions.add,
        clock,
        ctx,
    );

    event::emit(RoleCreated {
        target_key: self.target_key,
        role: copy role,
    });

    vec_map::insert(&mut self.roles, role, permissions);
}

/// Delete an existing role
public fun delete_role<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    role: &String,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.is_capability_valid(
        cap,
        &self.role_admin_permissions.delete,
        clock,
        ctx,
    );

    assert!(*role != self.initial_admin_role_name, EInitialAdminRoleCannotBeDeleted);
    vec_map::remove(&mut self.roles, role);

    event::emit(RoleRemoved {
        target_key: self.target_key,
        role: *role,
    });
}

/// Update permissions associated with an existing role
public fun update_role_permissions<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    role: &String,
    new_permissions: VecSet<P>,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.is_capability_valid(
        cap,
        &self.role_admin_permissions.update,
        clock,
        ctx,
    );

    if (*role == self.initial_admin_role_name) {
        assert!(
            has_required_admin_permissions(
                &new_permissions,
                &self.role_admin_permissions,
                &self.capability_admin_permissions,
            ),
            EInitialAdminPermissionsInconsistent,
        );
    };

    assert!(vec_map::contains(&self.roles, role), ERoleDoesNotExist);
    vec_map::remove(&mut self.roles, role);
    vec_map::insert(&mut self.roles, *role, new_permissions);

    event::emit(RoleUpdated {
        target_key: self.target_key,
        role: *role,
    });
}

/// Indicates if the specified role exists in the role_map
public fun has_role<P: copy + drop>(self: &RoleMap<P>, role: &String): bool {
    vec_map::contains(&self.roles, role)
}

/// ===== Capability Functions =======
/// Indicates if a provided capability is valid.
///
/// A capability is considered valid if:
/// - The capability's target_key matches the RoleMap's target_key.
///   Aborts with ECapabilitySecurityVaultIdMismatch if not matching.
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
/// - self: Reference to the `RoleMap` mapping.
/// - cap: Reference to the capability to be validated.
/// - permission: The permission to check against the capability's role.
/// - clock: Reference to a Clock instance for time-based validation.
/// - ctx: Reference to the transaction context for accessing the caller's address.
///
/// Returns
/// -------
/// - bool: true if the capability is valid, otherwise aborts with the relevant error.
public fun is_capability_valid<P: copy + drop>(
    self: &RoleMap<P>,
    cap: &Capability,
    permission: &P,
    clock: &Clock,
    ctx: &mut TxContext,
): bool {
    assert!(self.target_key == cap.target_key(), ECapabilitySecurityVaultIdMismatch);

    let permissions = self.get_role_permissions(cap.role());
    assert!(vec_set::contains(permissions, permission), ECapabilityPermissionDenied);

    assert!(self.issued_capabilities.contains(&cap.id()), ECapabilityHasBeenRevoked);

    if (cap.valid_from().is_some() || cap.valid_until().is_some()) {
        assert!(cap.is_currently_valid(clock), ECapabilityTimeConstraintsNotMet);
    };

    if (cap.issued_to().is_some()) {
        let caller = ctx.sender();
        let issued_to_addr = cap.issued_to().borrow();
        assert!(*issued_to_addr == caller, ECapabilityIssuedToMismatch);
    };

    true
}

/// Create a new capability
///
/// Parameters
/// ----------
/// - role_map: Reference to the `RoleMap` mapping.
/// - cap: Reference to the capability used to authorize the creation of the new capability.
/// - role: The role to be assigned to the new capability.
/// - issued_to: Optional address restriction for the new capability.
/// - valid_from: Optional start time (in milliseconds since Unix epoch) for the new capability.
/// - valid_until: Optional. Last point in time where the capability is valid (in milliseconds since Unix epoch).
/// - clock: Reference to a Clock instance for time-based validation.
/// - ctx: Reference to the transaction context.
///
/// Returns the newly created capability.
///
/// Sends a `CapabilityIssued` event upon successful creation.
///
/// Errors:
/// - Aborts with any error documented by `is_capability_valid` if the provided capability fails authorization checks.
/// - Aborts with `ERoleDoesNotExist` if the specified role does not exist in the role_map.
/// - Aborts with `tf_components::capability::EValidityPeriodInconsistent` if the provided valid_from and valid_until are inconsistent.
public fun new_capability<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    role: &String,
    issued_to: Option<address>,
    valid_from: Option<u64>,
    valid_until: Option<u64>,
    clock: &Clock,
    ctx: &mut TxContext,
): Capability {
    self.is_capability_valid(
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
    issue_capability(self, &new_cap);
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
public fun destroy_capability<P: copy + drop>(self: &mut RoleMap<P>, cap_to_destroy: Capability) {
    assert!(self.target_key == cap_to_destroy.target_key(), ECapabilitySecurityVaultIdMismatch);
    assert!(
        !self.initial_admin_cap_ids.contains(&cap_to_destroy.id()),
        EInitialAdminCapabilityMustBeExplicitlyDestroyed,
    );

    if (self.issued_capabilities.contains(&cap_to_destroy.id())) {
        self.issued_capabilities.remove(&cap_to_destroy.id());
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
/// Errors:
/// - Aborts with any error documented by `is_capability_valid` if the provided capability fails authorization checks.
/// - Aborts with `ECapabilityNotIssued` if `cap_to_revoke` is not currently issued by this `RoleMap`.
/// - Aborts with `EInitialAdminCapabilityMustBeExplicitlyDestroyed` if `cap_to_revoke` is an initial admin capability.
public fun revoke_capability<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    cap_to_revoke: ID,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.is_capability_valid(
        cap,
        &self.capability_admin_permissions.revoke,
        clock,
        ctx,
    );

    assert!(self.issued_capabilities.contains(&cap_to_revoke), ECapabilityNotIssued);
    assert!(
        !self.initial_admin_cap_ids.contains(&cap_to_revoke),
        EInitialAdminCapabilityMustBeExplicitlyDestroyed,
    );
    self.issued_capabilities.remove(&cap_to_revoke);

    event::emit(CapabilityRevoked {
        target_key: self.target_key,
        capability_id: cap_to_revoke,
    });
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
/// - Aborts with `ECapabilitySecurityVaultIdMismatch` if the capability's target_key does not match.
/// - Aborts with `ECapabilityIsNotInitialAdmin` if the capability is not an initial admin capability.
public fun destroy_initial_admin_capability<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap_to_destroy: Capability,
) {
    assert!(self.target_key == cap_to_destroy.target_key(), ECapabilitySecurityVaultIdMismatch);
    assert!(
        self.initial_admin_cap_ids.contains(&cap_to_destroy.id()),
        ECapabilityIsNotInitialAdmin,
    );

    self.issued_capabilities.remove(&cap_to_destroy.id());
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
/// Errors:
/// - Aborts with any error documented by `is_capability_valid` if the provided capability fails authorization checks.
/// - Aborts with `ECapabilityNotIssued` if `cap_to_revoke` is not currently issued by this `RoleMap`.
/// - Aborts with `ECapabilityIsNotInitialAdmin` if `cap_to_revoke` is not an initial admin capability.
public fun revoke_initial_admin_capability<P: copy + drop>(
    self: &mut RoleMap<P>,
    cap: &Capability,
    cap_to_revoke: ID,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.is_capability_valid(
        cap,
        &self.capability_admin_permissions.revoke,
        clock,
        ctx,
    );

    assert!(self.issued_capabilities.contains(&cap_to_revoke), ECapabilityNotIssued);
    assert!(self.initial_admin_cap_ids.contains(&cap_to_revoke), ECapabilityIsNotInitialAdmin);

    self.issued_capabilities.remove(&cap_to_revoke);
    self.initial_admin_cap_ids.remove(&cap_to_revoke);

    event::emit(CapabilityRevoked {
        target_key: self.target_key,
        capability_id: cap_to_revoke,
    });
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
fun issue_capability<P: copy + drop>(self: &mut RoleMap<P>, new_cap: &Capability) {
    self.issued_capabilities.insert(new_cap.id());
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

// =============== Getter Functions ======================

/// Returns the size of the role_map, the number of managed roles
public fun size<P: copy + drop>(self: &RoleMap<P>): u64 {
    vec_map::size(&self.roles)
}

/// Returns the target_key associated with the role_map
public fun target_key<P: copy + drop>(self: &RoleMap<P>): ID {
    self.target_key
}

//Returns the role admin permissions associated with the role_map
public fun role_admin_permissions<P: copy + drop>(self: &RoleMap<P>): &RoleAdminPermissions<P> {
    &self.role_admin_permissions
}

public fun issued_capabilities<P: copy + drop>(self: &RoleMap<P>): &VecSet<ID> {
    &self.issued_capabilities
}
