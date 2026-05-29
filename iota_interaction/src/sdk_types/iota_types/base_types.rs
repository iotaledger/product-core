// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::{TryFrom};
use std::fmt;
use std::result::Result::Ok;
use std::str::FromStr;

use anyhow::{anyhow};
use fastcrypto::hash::HashFunction;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use Result;

use super::{
    MOVE_STDLIB_ADDRESS,
    crypto::{
        AuthorityPublicKeyBytes, DefaultHash, IotaPublicKey, PublicKey,
    },
    iota_serde::to_iota_struct_tag_string,
    object::{Object, Owner},
    parse_iota_struct_tag,
};

use crate::move_core_types::account_address::AccountAddress;
use crate::move_core_types::identifier::IdentStr;
pub use super::digests::{ObjectDigest, TransactionDigest};
use crate::ident_str;

pub use iota_sdk_types::{
    Identifier, MoveObjectType, StructTag, TypeTag,
    ObjectId as ObjectID, ObjectReference as ObjectRef, Version as SequenceNumber,
};

// -----------------------------------------------------------------
// Originally defined in crates/iota-types/src/committee.rs
// -----------------------------------------------------------------
pub type EpochId = u64;
// TODO: the stake and voting power of a validator can be different so
// in some places when we are actually referring to the voting power, we
// should use a different type alias, field name, etc.
pub type StakeUnit = u64;
// -----------------------------------------------------------------
// Originally defined in crates/iota-types/src/execution_status.rs
// -----------------------------------------------------------------
pub type CommandIndex = usize;
// -----------------------------------------------------------------
// Originally defined in external-crates/move/crates/move-binary-format/src/file_format.rs
// -----------------------------------------------------------------
/// Index into the code stream for a jump. The offset is relative to the
/// beginning of the instruction stream.
pub type CodeOffset = u16;
/// Type parameters are encoded as indices. This index can also be used to
/// lookup the kind of a type parameter in the `FunctionHandle` and
/// `StructHandle`.
pub type TypeParameterIndex = u16;
// -----------------------------------------------------------------

pub type TxSequenceNumber = u64;

pub type VersionNumber = SequenceNumber;

/// The round number.
pub type CommitRound = u64;

pub type AuthorityName = AuthorityPublicKeyBytes;

/// Type of an IOTA object
#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ObjectType {
    /// Move package containing one or more bytecode modules
    Package,
    /// A Move struct of the given type
    Struct(MoveObjectType),
}

const PACKAGE: &str = "package";

impl ObjectType {
    pub fn is_gas_coin(&self) -> bool {
        matches!(self, ObjectType::Struct(s) if s.is_gas_coin())
    }

    pub fn is_coin(&self) -> bool {
        matches!(self, ObjectType::Struct(s) if s.is_coin())
    }

    pub fn is_package(&self) -> bool {
        matches!(self, ObjectType::Package)
    }
}

impl From<&Object> for ObjectType {
    fn from(o: &Object) -> Self {
        o.data
            .object_type()
            .map(|t| ObjectType::Struct(t.clone()))
            .unwrap_or(ObjectType::Package)
    }
}

impl TryFrom<ObjectType> for StructTag {
    type Error = anyhow::Error;

    fn try_from(o: ObjectType) -> Result<Self, anyhow::Error> {
        match o {
            ObjectType::Package => Err(anyhow!("Cannot create StructTag from Package")),
            ObjectType::Struct(s) => Ok(s.into()),
        }
    }
}

impl FromStr for ObjectType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == PACKAGE {
            Ok(ObjectType::Package)
        } else {
            let tag = parse_iota_struct_tag(s)?;
            Ok(ObjectType::Struct(tag.into()))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ObjectInfo {
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub type_: ObjectType,
    pub owner: Owner,
    pub previous_transaction: TransactionDigest,
}

impl ObjectInfo {
    pub fn new(oref: &ObjectRef, o: &Object) -> Self {
        Self {
            object_id: oref.object_id,
            version: oref.version,
            digest: oref.digest,
            type_: o.into(),
            owner: o.owner,
            previous_transaction: o.previous_transaction,
    }
    }

    pub fn from_object(object: &Object) -> Self {
        Self {
            object_id: object.id(),
            version: object.version(),
            digest: object.digest(),
            type_: object.into(),
            owner: object.owner,
            previous_transaction: object.previous_transaction,
    }
    }
}

impl From<ObjectInfo> for ObjectRef {
    fn from(info: ObjectInfo) -> Self {
        ObjectRef::new(info.object_id, info.version, info.digest)
    }
}

impl From<&ObjectInfo> for ObjectRef {
    fn from(info: &ObjectInfo) -> Self {
        ObjectRef::new(info.object_id, info.version, info.digest)
    }
}

pub const IOTA_ADDRESS_LENGTH: usize = ObjectID::LENGTH;

pub use iota_sdk_types::Address as IotaAddress;

pub fn address_from_iota_pub_key<T: IotaPublicKey>(pk: &T) -> IotaAddress {
        let mut hasher = DefaultHash::default();
        T::SIGNATURE_SCHEME.update_hasher_with_flag(&mut hasher);
        hasher.update(pk);
        let g_arr = hasher.finalize();
    IotaAddress::new(g_arr.digest)
}

impl From<&PublicKey> for IotaAddress {
    fn from(pk: &PublicKey) -> Self {
        let mut hasher = DefaultHash::default();
        pk.scheme().update_hasher_with_flag(&mut hasher);
        hasher.update(pk);
        let g_arr = hasher.finalize();
        IotaAddress::new(g_arr.digest)
    }
}

/// Generate a fake IotaAddress with repeated one byte.
pub fn dbg_addr(name: u8) -> IotaAddress {
    let addr = [name; IOTA_ADDRESS_LENGTH];
    IotaAddress::new(addr)
}

pub const RESOLVED_STD_OPTION: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    ident_str!("option"),
    ident_str!("Option"),
);

pub const RESOLVED_ASCII_STR: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    ident_str!("ascii"),
    ident_str!("String"),
);

pub const RESOLVED_UTF8_STR: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    ident_str!("string"),
    ident_str!("String"),
);

/// Generate a fake ObjectID with repeated one byte.
pub fn dbg_object_id(name: u8) -> ObjectID {
    ObjectID::new([name; ObjectID::LENGTH])
}

#[derive(PartialEq, Eq, Clone, Debug, thiserror::Error)]
pub enum ObjectIDParseError {
    #[error("ObjectID hex literal must start with 0x")]
    HexLiteralPrefixMissing,

    #[error("Could not convert from bytes slice")]
    TryFromSlice,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Package => write!(f, "{PACKAGE}"),
            ObjectType::Struct(t) => write!(
                f,
                "{}",
                to_iota_struct_tag_string(t).map_err(fmt::Error::custom)?
            ),
        }
    }
}