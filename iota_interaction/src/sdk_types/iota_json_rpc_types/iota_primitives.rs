// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! JSON Schema adapter types for the IOTA JSON-RPC surface, applied at field
//! sites via `#[serde_as(as = "...")]`. Core serde behaviour lives in
//! `iota_types::iota_serde`; this module adds the `schemars::JsonSchema` layer
//! on top (the `iota-types` crate intentionally has no `schemars` dependency).
//!
//! To add a new adapter, prefer a unit marker struct with a manual
//! `JsonSchema` impl for explicit control over description, format, and shape.
//! If custom serialisation is needed, delegate `SerializeAs` / `DeserializeAs`
//! to the corresponding adapter in `iota_types::iota_serde` so the two crates
//! cannot drift. Newtype wrappers (e.g. `SequenceNumberString(u64)`) are only
//! appropriate when the wrapper itself is the serialised value.

use crate::types::{
    self as iota_types,
    base_types::{StructTag as NativeStructTag, TypeTag as NativeTypeTag},
    iota_serde::{IotaStructTag, IotaTypeTag},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, DisplayFromStr, SerializeAs, serde_as};

/// A schema type that defines the JSON representation of the
/// [`IotaAddress`](iota_types::base_types::IotaAddress) type.
pub struct IotaAddress;

/// A schema type that defines the JSON representation of the
/// [`ObjectID`](iota_types::base_types::ObjectID) type.
pub struct ObjectID;

/// A schema type that defines the JSON representation of the
/// [`SequenceNumber`](iota_types::base_types::SequenceNumber) type as a string
/// and provides an alternate serialization usable via `#[serde_as]`.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SequenceNumberString(#[serde_as(as = "DisplayFromStr")] u64);

impl SerializeAs<iota_types::base_types::SequenceNumber> for SequenceNumberString {
    fn serialize_as<S>(
        source: &iota_types::base_types::SequenceNumber,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SequenceNumberString(source.as_u64()).serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, iota_types::base_types::SequenceNumber> for SequenceNumberString {
    fn deserialize_as<D>(
        deserializer: D,
    ) -> Result<iota_types::base_types::SequenceNumber, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = SequenceNumberString::deserialize(deserializer)?;
        Ok(iota_types::base_types::SequenceNumber::from_u64(schema.0))
    }
}

/// A schema type that defines the JSON representation of the
/// [`SequenceNumber`](iota_types::base_types::SequenceNumber) type as a u64
/// integer and uses the default serialization.
pub struct SequenceNumberU64;

/// A schema type that defines the JSON representation of the
/// [`ProtocolVersion`](iota_protocol_config::ProtocolVersion) type as a string
/// and provides an alternate serialization usable via `#[serde_as]`.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ProtocolVersion(
    #[serde_as(as = "DisplayFromStr")]
    u64,
);

/// A schema type that defines the JSON representation of a Base58 encoded
/// string. A custom JsonSchema impl is necessary to add the "base58" format to
/// the schema.
pub struct Base58;

/// A schema type that defines the JSON representation of a Base64 encoded
/// string. A custom JsonSchema impl is necessary to add the "base64" format to
/// the schema.
pub struct Base64;

/// A schema type that defines the JSON representation of a Base64 encoded
/// signature.
pub struct GenericSignature;

/// A schema type that defines the JSON representation of a Move
/// [`StructTag`](iota_types::base_types::StructTag) as a string, and
/// provides a string serialization usable via `#[serde_as]`.
pub struct StructTag;

impl SerializeAs<NativeStructTag> for StructTag {
    fn serialize_as<S>(value: &NativeStructTag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        IotaStructTag::serialize_as(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, NativeStructTag> for StructTag {
    fn deserialize_as<D>(deserializer: D) -> Result<NativeStructTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        IotaStructTag::deserialize_as(deserializer)
    }
}

/// A schema type that defines the JSON representation of a Move
/// [`TypeTag`](iota_types::base_types::TypeTag) as a string, and
/// provides a string serialization usable via `#[serde_as]`.
pub struct TypeTag;

impl SerializeAs<NativeTypeTag> for TypeTag {
    fn serialize_as<S>(value: &NativeTypeTag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        IotaTypeTag::serialize_as(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, NativeTypeTag> for TypeTag {
    fn deserialize_as<D>(deserializer: D) -> Result<NativeTypeTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        IotaTypeTag::deserialize_as(deserializer)
    }
}

/// A schema type that defines the JSON representation of a Move identifier,
/// and provides a string serialization usable via `#[serde_as]`.
pub struct Identifier;
