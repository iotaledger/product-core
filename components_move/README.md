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
The `role_map` module directly depends on the `tf_components::capability` module. Both modules are tight strongly together.

### Usage Examples

The [`Counter` example](./examples/counter/README.md) is a very simple example, demonstrating how to use
`RoleMap` and `Capability` for role based access control. The accompanying 
[test file](./examples/counter/tests/counter_tests.move) demonstrates the Move user experience.

The TF product Audit Trails uses the `RoleMap` to manage access to the audit trail records and their operations, which 
can be seen as a more complex example:
* The `RoleMap` is integrated in the `audit_trail::main` module to manage access to the audit trail records and 
  their operations. See [here](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L208) for an example.
* The `RoleMap` is created by the `AuditTrail` in it's [create function](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/audit_trail.move#L114).
* An example for the Move user experience can be found in the [capability_tests.move](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/tests/capability_tests.move) file.
