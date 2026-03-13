# IOTA Move POC - Borrowing "non store" Child Objects from Parents

This proof-of-concept demonstrates how to transfer an owned object (the “child”) not haviing `store` ability, to a parent address and then safely “receive” 
it inside a different module using IOTA Move’s `Receiving<T>` pattern. It also shows how to mutate the received child and return it back to the parent’s address.

It also provides an enum `BorrowedChild` located in the `TfComponents` package which can be used to abstract away multiple types of child objects that shall be 
borrowed from their parent objects where childs have been send to their parents as been described
[here](https://docs.iota.org/developer/iota-101/objects/transfers/transfer-to-object#transferring-to-an-object).

This scenario is needed in case objects, not having `store` ability, shall be borrowed by other objects (parents).

The `BorrowedChild` enum helps to collect all child specific dependencies in the `borrowed_child` module so that parent object implementations only need to depend on `borrowed_child`.

## Motivation for the `BorrowedChild` enum

The problem:
* Usually in a scenario with `P` parent types and `C` child types, all `P` parent modules would need to implement the child specific transfers, receives etc. for `C` child types in their modules. This results in `P` * `C` almost redundant implementations.
* Every time a new child type is added all `P` parent modules would need to be extended accordingly.

How `tf_components::borrowed_child::BorrowedChild` solves this:
* All child specific transfers, receives etc. actions are implemented in the `borrowed_child` module.
* Instead of `P` * `C` implementations of child specific code, only
   * P implementations of `BorrowedChild` specific code and
   * One `BorrowedChild` implementation for the `C` child types are needed
* Users access client specific types using functions provided by the `borrowed_child` module:
   1. Get  `BorrowedChild` from parent module
   2. Extract `BorrowedChild` into child type and a `Pledge` (hot potato) using  `borrowed_child` module functions
   3. Do something with child in the PTB
   4. Restore the  `BorrowedChild` using child specific functions provided by the `borrowed_child` module (`Pledge` will be needed for this)
   5. Put back the  `BorrowedChild` to the parent

To see the `parent::borrow_child()` and `put_back()` function in action have a look into the test `test_borrow_child_and_put_back()` in the `tests/parent_tests.move` file.

### What this POC shows

- **Creating** a child object (`tf_components::example_child::create`).
- **Creating** a parent object (`borrow_non_store_obj_poc::parent::create`).
- **Transferring** the child object to the parent’s address (`tf_components::example_child::transfer_object`).
- **Borrowing** the child in a PTB (`borrow_non_store_obj_poc::parent::borrow_child` / `put_back`) via `tf_components::borrowed_child`.
- **Mutating** the child (increment a counter)

---

## Quick start

1) Make scripts executable

```bash
chmod +x deploy.sh poc.sh
```

2) Build and publish both packages

```bash
./deploy.sh
```

The script prints two IDs:

- `Child Package ID: <0x...>`
- `Parent Package ID: <0x...>`

3) Update `poc.sh`
Open `poc.sh` and set:

```bash
TF_COMP_PACKAGE"0x..."
PARENT_PACKAGE="0x..."
```

Use the package ids printed by `deploy.sh`at the back of log.

4) Run the POC

```bash
./poc.sh
```

You should see logs for creating objects, transferring the child, receiving + incrementing, and finally a printed counter value.

---

## What `poc.sh` does (step-by-step)

The script drives a single PTB-based flow:

1) **Create the child**
   - Calls `tf_components::example_child::create` to mint an `ExampleChildObject` to your sender address.
   - Captures the new child’s `objectId` from JSON output.

2) **Create the parent**
   - Calls `borrow_non_store_obj_poc::parent::create` to mint an `ExampleParentObject` to your sender address.
   - Captures the parent’s `objectId`.

3) **Transfer the child to the parent’s address**
   - Calls `tf_components::example_child::transfer_object(child, parent_addr)`.
   - The script passes the child `objectId` and the parent `objectId` (treated as an address). The `@` prefix in CLI args indicates an object ID/address literal.

4) **Borrow the child, increment the counter inside the child and put it back to the parent**
   The script Calls: 
   - `borrow_non_store_obj_poc::parent::borrow_child(&mut parent, request: tf_components::borrowed_child:::BorrowRequest): BorrowedChild`.
      - The `BorrowRequest` contains a `Receiving<ExampleChildObject>` which is used to receive the child
      - The function returns a `BorrowedChild`instance
   - `tf_components::borrowed_child::extract_example(BorrowedChild)`
      - Extracts the `ExampleChildObject` from the `BorrowedChild` and returns it together with a `Pledge` (hot potato).
   - `tf_components::example_child::increment(&mut child)`
   - `tf_components::borrowed_child::example(ExampleChildObject, Pledge): BorrowedChild`
      - Creates a new `BorrowedChild` instance from the mutated `ExampleChildObject` and the `Pledge`
   - `borrow_non_store_obj_poc::parent::put_back(&mut parent, BorrowedChild)`
      - Puts back the child to the parent by consuming the `ExampleChildObject` and the `Pledge` contained in the `BorrowedChild`
      - The `put_back` function ensures that the child is transferred back to the parent’s address
5) **Read the counter**
   - The ExampleChildObject is borrowed from the parent in the same way as step 4, but instead of incrementing the counter, it just calls `tf_components::example_child::get_counter(&child)`.
     - The value reeturned from `get_counter` is assigned to a global variable `COUNTER`
   - The final printed value should be the child’s counter after the increment.
     ATM the `COUNTER` value is undefined. Probably this is caused by the `iota client ptb --assign` function which seems to be only able to assign
     values on PTB scope but not on console scope. This is something to be investigated in the future.
---

## How the receiving pattern works

- The `Receiving<T>` type represents a to-be-received object that was transferred to an address but not yet materialized under a specific object’s authority.
- The parent completes the receive with `tf_components::example_child::receive(&mut parent.id, r)` which checks the receiver and returns an `ExampleChildObject` for in-module use.
- After you’re done, you can re-transfer the child back to the parent’s address to maintain ownership consistency.

---

## Troubleshooting

- **Object not found / wrong object type**: Ensure you used the correct package IDs and the latest `child` and `parent` object IDs from the creation steps.
- **Insufficient gas**: Increase `--gas-budget`.
- **CLI errors**: Verify your active address/network with `iota client switch` and ensure `jq` is installed.
