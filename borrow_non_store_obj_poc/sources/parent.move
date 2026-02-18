// For Move coding conventions, see
// https://docs.iota.org/developer/iota-101/move-overview/conventions
module borrow_non_store_obj_poc::parent;

// These dependencies are only needed if ExampleChildObject is directly used in the parent interface
//use tf_components::example_child::{ExampleChildObject};
//use iota::transfer::Receiving;

use tf_components::borrowed_child::{Self, BorrowRequest, BorrowedChild};

// ------------------------------------------------------------------------------------

public struct ExampleParentObject has key {
    id: object::UID,
}

public fun create(ctx: &mut TxContext) {
    let s = ExampleParentObject {
        id: object::new(ctx),
    };
    transfer::transfer(s, ctx.sender());
}

public fun borrow_child(    
    obj: &mut ExampleParentObject,
    request: BorrowRequest
): BorrowedChild{
    borrowed_child::borrow(&mut obj.id, request)
}

public fun put_back(obj: &mut ExampleParentObject, borrowed_obj: BorrowedChild) {
    borrowed_child::put_back(obj.id.as_inner(), borrowed_obj)
}

// If we need to, we can use a dependency like ExampleChildObject directly in the parent interface.
/*
public fun receive_child(
    obj: &mut ExampleParentObject,
    receiver: Receiving<ExampleChildObject>,
): ExampleChildObject {
    child::child_object::receive(&mut obj.id, receiver)
}

public fun receive_increment_child(
    obj: &mut ExampleParentObject,
    r: Receiving<ExampleChildObject>,
) {
    let mut c = child::child_object::receive(&mut obj.id, r);
    child::child_object::increment(&mut c);
    child::child_object::transfer_object(c, obj.id.to_address());
}

public fun borrow_example_child(
    obj: &mut ExampleParentObject,
    receiver: Receiving<ExampleChildObject>,
): (borrowed_child::Pledge, ExampleChildObject) {
    borrowed_child::borrow(&mut obj.id, borrowed_child::request_example(receiver))
        .extract_example()
}

public fun put_back_example_child(parent_object: &mut ExampleParentObject, child: ExampleChildObject, pledge: borrowed_child::Pledge) {
    borrowed_child::put_back(parent_object.id.as_inner(), borrowed_child::example(pledge, child));
}
*/

