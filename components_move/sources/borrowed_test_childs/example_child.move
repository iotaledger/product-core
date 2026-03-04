module tf_components::example_child;

use iota::transfer::Receiving;

public struct ExampleChildObject has key {
    id: object::UID,
    counter: u64,
}

public fun create(ctx: &mut TxContext) {
    let s = ExampleChildObject {
        id: object::new(ctx),
        counter: 0,
    };
    transfer::transfer(s, ctx.sender());
}

public fun get_counter(obj: &ExampleChildObject): u64 {
    obj.counter
}

/// Anyone can increment the counter in the owned object.
public fun increment(obj: &mut ExampleChildObject) {
    obj.counter = obj.counter + 1;
}

/// Owned objects can be transferred to any address.
public fun transfer_object(obj: ExampleChildObject, receiver: address) {
    transfer::transfer(obj, receiver);
}

public fun receive(obj: &mut UID, receiver: Receiving<ExampleChildObject>): ExampleChildObject {
    transfer::receive(obj, receiver)
}
