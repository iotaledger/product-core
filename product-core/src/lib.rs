// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::Address;

pub mod move_repr;
pub mod move_type;
pub mod network;
pub mod operation;
pub mod product_client;

pub const CLOCK_ADDRESS: Address = address_from_u8(6);

const fn address_from_u8(value: u8) -> Address {
    let mut bytes = [0; Address::LENGTH];
    *bytes.last_mut().unwrap() = value;

    Address::new(bytes)
}