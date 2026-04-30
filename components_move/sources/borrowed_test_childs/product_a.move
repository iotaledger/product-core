module tf_components::product_a;

use iota::transfer::Receiving;

public struct ProductA has key {
    id: object::UID,
}

public fun transfer_object(obj: ProductA, receiver: address) {
    transfer::transfer(obj, receiver);
}

public fun receive(obj: &mut UID, receiver: Receiving<ProductA>): ProductA {
    transfer::receive(obj, receiver)
}