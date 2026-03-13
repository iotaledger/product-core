// A minimal shared object that can also be used as a parent for borrowing child objects but has severe security issues.
//
// Unlike the `parent::ExampleParentObject`, `SomeOtherParent` doesn't provide a `borrow_child()` function.
// Instead `SomeOtherParent` offers a `uid_mut()` function (not recommended).
module borrow_non_store_obj_poc::other_parent;

public struct SomeOtherParent has key {
    id: object::UID,
}

public fun create(ctx: &mut TxContext) {
    let other_parent = SomeOtherParent {
        id: object::new(ctx),
    };
    transfer::share_object(other_parent);
}

/// Providing the id as `&mut UID` without any restriction, can be considered an anti pattern
/// or at least a major security risk.
///
/// We use this here to allow borrowing child objects from `SomeOtherParent` without having a dedicated `borrow_child()` function.
/// Because the `&mut UID` is publicly available, `SomeOtherParent` looses all control over it's child objects.
/// Users can not only borrow child objects, they can also steal them.
public fun uid_mut(self: &mut SomeOtherParent): &mut UID {
    &mut self.id
}
