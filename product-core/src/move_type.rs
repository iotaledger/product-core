// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::TypeTag;

use crate::product_client::{Product, ProductClient};

pub trait MoveType {
    type Product: Product;
    fn move_type(client: &impl ProductClient<Self::Product>) -> TypeTag;
}
