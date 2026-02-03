use iota_sdk::types::TypeTag;

use crate::product_client::{Product, ProductClient};

pub trait MoveType {
    type Product: Product;
    fn move_type(client: &impl ProductClient<Self::Product>) -> TypeTag;
}
