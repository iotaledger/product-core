module tf_components::product_b;

use iota::transfer::Receiving;

public struct ProductB has key {
    id: object::UID,
}

public fun transfer_object(obj: ProductB, receiver: address) {
    transfer::transfer(obj, receiver);
}

public fun receive(obj: &mut UID, receiver: Receiving<ProductB>): ProductB {
    transfer::receive(obj, receiver)
}