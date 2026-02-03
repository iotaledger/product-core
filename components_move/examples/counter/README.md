# RoleMap Integration Example

This Counter example shows how the `role_map::RoleMap` can be integrated into 3rd party shared objects (or Trust Framework products). The example integrates the `role_map` and `capability` modules into a simple shared `Counter` object, as being described
[here](https://docs.iota.org/developer/iota-101/move-overview/package-upgrades/upgrade#4-guard-function-access).

## Permissions

In general, to integrate the `RoleMap` into a shared object,
we need to define a Permission enum similar to the [enum used for IOTA Audit Trails](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/permission.move#L10-L11).

The Permission enum for the shared `Counter` example is called `CounterPermission` and 
can be found in the [permission.move](./sources/permission.move) file.

## RoleMap Integration

The `RoleMap` is used to manage access control to target objects. In this example a target object is a `Counter` object. The integration of the `RoleMap<CounterPermission>` into the target object type (the `Counter` struct), requires the following steps (implementation of the shared `Counter` example can be found in [counter.move](./sources/counter.move)):

* instantiate the `RoleMap` instance in its `create()` function
* provide the necessary getter and mutator functions for users to access the `RoleMap` instance
  (function `access()` and `access_mut()` in [counter.move](./sources/counter.move))
* use the `RoleMap.is_capability_valid()` function to check whether a provided capability has the required permission
  (used in function `increment()` in [counter.move](./sources/counter.move))

## `RoleMap` User experience and testing the Integration

The accompanying [counter_tests.move](./tests/counter_tests.move) file demonstrates the
user experience of Move users interacting with the shared `Counter` object via the integrated
`RoleMap` and `Capability` objects.

To run the tests, run the following command in the `TfComponents` package root directory:

```bash
iota move test
```