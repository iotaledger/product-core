# Trust Framework Components

The `TfComponents` package contains Move modules providing reusable components for building smart contracts
within the IOTA Trust Framework. These components are designed to be modular and easily integrated into various
Trust Framework products and community-developed smart contracts.

Modules Overview:
* `role_map`:   Implements the `RoleMap` struct for role-based access control, allowing mapping of roles to 
                application specific permissions.
* `capability`: Defines the `Capability` struct for granting access rights within smart contracts in conjunction with 
                the `RoleMap<P>` struct.
* `timelock`:   Provides the `Timelock` struct for expressing and processing time-based restrictions

## Role-Based Access Control - The `role_map` and the `capability` Module

The `role_map` module provides the `RoleMap<P>` struct, which is 
a role-based access control helper that maps unique role identifiers to their associated permissions.

The `capability` module provides the `capability::Capability` struct,
a capability token that grants access rights defined by one specific role in the `RoleMap`.

The `role_map` module directly depends on the `capability` module. Both modules are tight strongly together.

The `RoleMap<P>` struct provides the following functionalities:
- Uses custom permission types defined by the integrating module using the generic argument `P`
- Defines an initial role with a custom set of permissions (i.e. for an Admin role) and creates an initial
  `Capability` for this role to allow later access control administration by the creator of the integrating module
- Allows to create, delete, and update roles and their permissions
- Allows to issue, revoke, and destroy `Capability`s associated with a specific role
- Validates `Capability`s against the defined roles to facilitate proper access control by the integrating module
  (function `RoleMap.is_capability_valid()`)
- All functions are access restricted by custom permissions defined during `RoleMap` instantiation

### Usage Examples

The [`Counter` example](./examples/counter/README.md) is a very simple example, demonstrating how to use
`RoleMap` and `Capability` for role based access control. The accompanying 
[test file](./examples/counter/tests/counter_tests.move) demonstrates the Move user experience.

The Trust Framework product *Audit Trails* uses the `RoleMap` to manage access to the audit trail records and their operations, which 
can be seen as a more complex example:
* The `RoleMap` is integrated in the `audit_trail::main` module to manage access to the audit trail records and 
  their operations. See [here](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L208) for an example.
* The `RoleMap` is created by the `AuditTrail` in it's [create function](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L114).
* An example for the Move user experience can be found in the [capability_tests.move](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/tests/capability_tests.move) file.
