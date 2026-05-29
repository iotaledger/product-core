// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[allow(unused)] // Kept in sync with original source, so keep as is.
use serde::{Deserialize, Serialize};
use super::base_types::{IotaAddress, ObjectID, SequenceNumber, StructTag, TypeTag};
pub use iota_sdk_types as sdk_types;
use crate::move_core_types::account_address::AccountAddress;
use super::object::OBJECT_START_VERSION;
use super::{
    iota_sdk_types_conversions::{struct_tag_core_to_sdk, type_tag_core_to_sdk},
};
macro_rules! built_in_ids {
    ($($addr:ident / $id:ident = $init:expr);* $(;)?) => {
        $(
            pub const $addr: AccountAddress = builtin_address($init);
            pub const $id: ObjectID = ObjectID::new($addr.into_bytes());
        )*
    }
}

macro_rules! built_in_pkgs {
    ($($addr:ident / $id:ident = $init:expr);* $(;)?) => {
        built_in_ids! { $($addr / $id = $init;)* }
    }
}

built_in_pkgs! {
    MOVE_STDLIB_ADDRESS / MOVE_STDLIB_PACKAGE_ID = 0x1;
    IOTA_FRAMEWORK_ADDRESS / IOTA_FRAMEWORK_PACKAGE_ID = 0x2;
    IOTA_SYSTEM_ADDRESS / IOTA_SYSTEM_PACKAGE_ID = 0x3;
    GENESIS_BRIDGE_ADDRESS / GENESIS_BRIDGE_PACKAGE_ID = 0xb;
    STARDUST_ADDRESS / STARDUST_PACKAGE_ID = 0x107a;
}

built_in_ids! {
    IOTA_SYSTEM_STATE_ADDRESS / IOTA_SYSTEM_STATE_OBJECT_ID = 0x5;
    IOTA_CLOCK_ADDRESS / IOTA_CLOCK_OBJECT_ID = 0x6;
    IOTA_AUTHENTICATOR_STATE_ADDRESS / IOTA_AUTHENTICATOR_STATE_OBJECT_ID = 0x7;
    IOTA_RANDOMNESS_STATE_ADDRESS / IOTA_RANDOMNESS_STATE_OBJECT_ID = 0x8;
    GENESIS_IOTA_BRIDGE_ADDRESS / GENESIS_IOTA_BRIDGE_OBJECT_ID = 0x9;
    IOTA_DENY_LIST_ADDRESS / IOTA_DENY_LIST_OBJECT_ID = 0x403;
}

pub const SYSTEM_PACKAGE_ADDRESSES: [IotaAddress; 5] = [
    IotaAddress::STD,
    IotaAddress::FRAMEWORK,
    IotaAddress::SYSTEM,
    IotaAddress::GENESIS_BRIDGE,
    IotaAddress::STARDUST,
];

pub const IOTA_SYSTEM_STATE_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;
pub const IOTA_CLOCK_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;
pub const IOTA_AUTHENTICATOR_STATE_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;

const fn builtin_address(suffix: u16) -> AccountAddress {
    let mut addr = [0u8; AccountAddress::LENGTH];
    let [hi, lo] = suffix.to_be_bytes();
    addr[AccountAddress::LENGTH - 2] = hi;
    addr[AccountAddress::LENGTH - 1] = lo;
    AccountAddress::new(addr)
}

/// Parse `s` as a struct type: A fully-qualified name, optionally followed by a
/// list of type parameters (types -- see `parse_iota_type_tag`, separated by
/// commas, surrounded by angle brackets). Parsing succeeds if and only if `s`
/// matches this format exactly, with no remaining input. This function is
/// intended for use within the authority codebase.
pub fn parse_iota_struct_tag(s: &str) -> anyhow::Result<StructTag> {
    use move_core_types::parsing::types::ParsedStructType;
    ParsedStructType::parse(s)?
        .into_struct_tag(&resolve_address)
        .map(|s| struct_tag_core_to_sdk(&s))
}

/// Parse `s` as a type: Either a struct type (see `parse_iota_struct_tag`), a
/// primitive type, or a vector with a type parameter. Parsing succeeds if and
/// only if `s` matches this format exactly, with no remaining input. This
/// function is intended for use within the authority codebase.
pub fn parse_iota_type_tag(s: &str) -> anyhow::Result<TypeTag> {
    use move_core_types::parsing::types::ParsedType;
    ParsedType::parse(s)?
        .into_type_tag(&resolve_address)
        .map(|s| type_tag_core_to_sdk(&s))
}

/// Resolve well-known named addresses into numeric addresses.
pub fn resolve_address(addr: &str) -> Option<AccountAddress> {
    match addr {
        "std" => Some(IotaAddress::STD),
        "iota" => Some(IotaAddress::FRAMEWORK),
        "iota_system" => Some(IotaAddress::SYSTEM),
        "stardust" => Some(IotaAddress::STARDUST),
        _ => None,
    }
    .map(|addr| AccountAddress::new(addr.into_bytes()))
}

pub trait MoveTypeTagTrait {
    fn get_type_tag() -> TypeTag;
}

impl MoveTypeTagTrait for u8 {
    fn get_type_tag() -> TypeTag {
        TypeTag::U8
    }
}

impl MoveTypeTagTrait for u64 {
    fn get_type_tag() -> TypeTag {
        TypeTag::U64
    }
}

impl MoveTypeTagTrait for ObjectID {
    fn get_type_tag() -> TypeTag {
        TypeTag::Address
    }
}

impl MoveTypeTagTrait for IotaAddress {
    fn get_type_tag() -> TypeTag {
        TypeTag::Address
    }
}

impl<T: MoveTypeTagTrait> MoveTypeTagTrait for Vec<T> {
    fn get_type_tag() -> TypeTag {
        TypeTag::Vector(Box::new(T::get_type_tag()))
    }
}
