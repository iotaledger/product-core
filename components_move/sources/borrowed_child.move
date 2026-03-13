/// Provides helper types to
///
/// All child specific dependencies are collected in this module
/// so that parent object implementations only need to depend on
/// the `borrowed_child` module.
module tf_components::borrowed_child;

use iota::transfer::Receiving;
use tf_components::example_child::{Self, ExampleChildObject};
use tf_components::product_a::{Self, ProductA};
use tf_components::product_b::{Self, ProductB};

// ===== Errors =====

/// Error when BorrowedChild variant does not match the choosen extract function
const EBorrowedChildVariantMismatch: u64 = 0;

// ==========================
// ===== BorrowRequest  =====
// ==========================

/// Needed to request a `BorrowedChild` from a parent object.
///
/// Parent objects offer a `borrow_child()` function like
/// ```
///    public fun borrow_child(request: BorrowRequest): BorrowedChild
/// ```
/// which will consume the `BorrowRequest`.

public enum BorrowRequest {
    A(Receiving<ProductA>),
    B(Receiving<ProductB>),
    Example(Receiving<ExampleChildObject>),
}

public fun request_a(receiver: Receiving<ProductA>): BorrowRequest {
    BorrowRequest::A(receiver)
}

public fun request_b(receiver: Receiving<ProductB>): BorrowRequest {
    BorrowRequest::B(receiver)
}

public fun request_example(receiver: Receiving<ExampleChildObject>): BorrowRequest {
    BorrowRequest::Example(receiver)
}

// ==========================
// ===== Pledge         =====
// ==========================

/// A hot potato making sure a `BorrowedChild` is put back once borrowed.
public struct Pledge {
    parent: ID,
    child: ID,
}

public fun pledge<C: key>(parent_id: &ID, child: &C): Pledge {
    Pledge {
        parent: *parent_id,
        child: object::id(child),
    }
}

public fun is_valid(pledge: &Pledge, parent_object_id: &ID, example_child_id: &ID): bool {
    parent_object_id == pledge.parent && example_child_id == pledge.child
}


// ==========================
// ===== BorrowedChild  =====
// ==========================

/// Wraps a parent object.
///
/// Parent objects offer a `borrow_child()` function like
/// ```
///    public fun borrow_child(request: BorrowRequest): (Pledge, BorrowedChild)
/// ```
/// which will consume the `BorrowRequest`.

public enum BorrowedChild {
    A(Pledge, ProductA),
    B(Pledge, ProductB),
    Example(Pledge, ExampleChildObject),
}

// --- Creating BorrowedChild instances -------------------

public fun a(pledge: Pledge, child: ProductA): BorrowedChild {
    BorrowedChild::A(pledge, child)
}

public fun b(pledge: Pledge, child: ProductB): BorrowedChild {
    BorrowedChild::B(pledge, child)
}

public fun example(pledge: Pledge, child: ExampleChildObject): BorrowedChild {
    BorrowedChild::Example(pledge, child)
}

// --- Extracting inner child + pledge and destroying BorrowedChild ---

public fun extract_a(borrowed: BorrowedChild): (Pledge, ProductA) {
    match (borrowed) {
        BorrowedChild::A(pledge, child) => (pledge, child),
        BorrowedChild::B(_pledge, _child) => abort EBorrowedChildVariantMismatch,
        BorrowedChild::Example(_pledge, _child) => abort EBorrowedChildVariantMismatch,
    }
}

public fun extract_b(borrowed: BorrowedChild): (Pledge, ProductB) {
    match (borrowed) {
        BorrowedChild::A(_pledge, _child) => abort EBorrowedChildVariantMismatch,
        BorrowedChild::B(pledge, child) => (pledge, child),
        BorrowedChild::Example(_pledge, _child) => abort EBorrowedChildVariantMismatch,
    }
}

public fun extract_example(borrowed: BorrowedChild): (Pledge, ExampleChildObject) {
    match (borrowed) {
        BorrowedChild::A(_pledge, _child) => abort EBorrowedChildVariantMismatch,
        BorrowedChild::B(_pledge, _child) => abort EBorrowedChildVariantMismatch,
        BorrowedChild::Example(pledge, child) => (pledge, child),
    }
}

// --- Borrow inner child form BorrowedChild --------------

public fun borrow_a(parent: &mut UID, receiver: Receiving<ProductA>): BorrowedChild {
    let child = product_a::receive(parent, receiver);
    BorrowedChild::A(pledge(parent.as_inner(), &child), child)
}

public fun borrow_b(parent: &mut UID, receiver: Receiving<ProductB>): BorrowedChild {
    let child = product_b::receive(parent, receiver);
    BorrowedChild::B(pledge(parent.as_inner(), &child), child)
}

public fun borrow_example(
    parent: &mut UID,
    receiver: Receiving<ExampleChildObject>,
): BorrowedChild {
    let child = example_child::receive(parent, receiver);
    BorrowedChild::Example(pledge(parent.as_inner(), &child), child)
}

// --- Type specific functions to put back a borrowed child ---

public fun put_back_a(parent_object_id: &ID, pledge: Pledge, child: ProductA) {
    assert!(pledge.is_valid(parent_object_id, &object::id(&child)));
    product_a::transfer_object(child, parent_object_id.to_address());
    let Pledge { parent: _, child: _ } = pledge;
}

public fun put_back_b(parent_object_id: &ID, pledge: Pledge, child: ProductB) {
    assert!(pledge.is_valid(parent_object_id, &object::id(&child)));
    product_b::transfer_object(child, parent_object_id.to_address());
    let Pledge { parent: _, child: _ } = pledge;
}

public fun put_back_example(parent_object_id: &ID, pledge: Pledge, child: ExampleChildObject) {
    assert!(pledge.is_valid(parent_object_id, &object::id(&child)));
    example_child::transfer_object(child, parent_object_id.to_address());
    let Pledge { parent: _, child: _ } = pledge;
}

// --- Borrow a BorrowedChild ---------------------------

public fun borrow(parent: &mut UID, request: BorrowRequest): BorrowedChild {
    match (request) {
        BorrowRequest::A(receiver) => borrow_a(parent, receiver),
        BorrowRequest::B(receiver) => borrow_b(parent, receiver),
        BorrowRequest::Example(receiver) => borrow_example(parent, receiver),
    }
}

// --- Put back a BorrowedChild ---------------------------

public fun put_back(parent_object_id: &ID, borrowed_obj: BorrowedChild) {
    match (borrowed_obj) {
        BorrowedChild::A(pledge, child) => put_back_a(parent_object_id, pledge, child),
        BorrowedChild::B(pledge, child) => put_back_b(parent_object_id, pledge, child),
        BorrowedChild::Example(pledge, child) => put_back_example(parent_object_id, pledge, child),
    };
}
