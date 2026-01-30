# RoleMap Integration Example

The Counter example shows how the `RoleMap` can be integrated into 3rd party shared objects
(or Trust Framework products). In this example the `role_map` and `capability` modules are 
integrated into a simple shared `Counter` object, as being described
[here](https://docs.iota.org/developer/iota-101/move-overview/package-upgrades/upgrade#4-guard-function-access).

## Permissions

In general, to integrate the `RoleMap` into a shared object,
we need to define a Permission enum similar to the [enum used for IOTA Audit Trails](https://github.com/iotaledger/notarization/blob/main/audit-trail-move/sources/permission.move#L10-L11).

The Permission enum for the shared `Counter` example is called `CounterPermission` and 
can be found in the [permission.move](./sources/permission.move) file.

## RoleMap Integration

The integration of the `RoleMap<CounterPermission>` into the target object, which is the `Counter` object in
this example, requires the following steps. The target object needs to:

* instantiate the `RoleMap` instance in its create function
* provide the necessary getter and mutator functions for users to access the `RoleMap` instance
* use the `RoleMap.is_capability_valid()` function to check whether a provided capability has the required permission

The implementation of the shared `Counter` example can be found in the
[counter.move](./sources/counter.move) file.

## User experience and testing the Integration

The accompanying [counter_tests.move](./tests/counter_tests.move) file demonstrates the
user experience of users interacting with the shared `Counter` object using the integrated
`RoleMap` and `Capability` objects.

To run the tests, use the following command from the `TfComponents` package root directory:

```bash
iota move test
```