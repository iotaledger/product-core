// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write};
use std::marker::{PhantomData, Sized};
use std::ops::Deref;
#[allow(unused)] // Kept in sync with original source, so keep as is.
use std::option::Option;
use std::option::Option::Some;
use std::result::Result::Ok;
use std::str::FromStr;
use std::string::{String, ToString};

use serde::de::{Deserializer, Error};
use serde::ser::{Error as SerError, Serializer};
use serde::{self, Deserialize, Serialize};
use serde_with::{serde_as, DeserializeAs, DisplayFromStr, SerializeAs};
use Result;

use super::{parse_iota_struct_tag, parse_iota_type_tag};
#[allow(unused)] // Kept in sync with original source, so keep as is.
use super::{
    base_types::{IotaAddress, StructTag, TypeTag},
};

/// The minimum and maximum protocol versions supported by this build.
const MIN_PROTOCOL_VERSION: u64 = 1;      // Originally defined in crates/iota-protocol-config/src/lib.rs
pub const MAX_PROTOCOL_VERSION: u64 = 26; // Originally defined in crates/iota-protocol-config/src/lib.rs

// -----------------------------------------------------------------------------------------
// Originally contained in crates/iota-protocol-config/src/lib.rs
// -----------------------------------------------------------------------------------------

// Record history of protocol version allocations here:
//
// Version 1:  Original version.
// Version 2:  Don't redistribute slashed staking rewards, fix computation of
//             SystemEpochInfoEventV1.
// Version 3:  Set the `relocate_event_module` to be true so that the module
//             that is associated as the "sending module" for an event is
//             relocated by linkage.
//             Add `Clock` based unlock to `Timelock` objects.
// Version 4:  Introduce the `max_type_to_layout_nodes` config that sets the
//             maximal nodes which are allowed when converting to a type layout.
// Version 5:  Introduce fixed protocol-defined base fee, IotaSystemStateV2 and
//             SystemEpochInfoEventV2.
//             Disallow adding new modules in `deps-only` packages.
//             Improve gas/wall time efficiency of some Move stdlib vector
//             functions.
//             Add new gas model version to update charging of functions.
//             Enable proper conversion of certain type argument errors in the
//             execution layer.
// Version 6:  Bound size of values created in the adapter.
// Version 7:  Improve handling of stake withdrawal from candidate validators.
// Version 8:  Variants as type nodes.
//             Enable smart ancestor selection for testnet.
//             Enable probing for accepted rounds in round prober for testnet.
//             Switch to distributed vote scoring in consensus in testnet.
//             Enable zstd compression for consensus tonic network in testnet.
//             Enable consensus garbage collection for testnet
//             Enable the new consensus commit rule for testnet.
//             Enable min_free_execution_slot for the shared object congestion
//             tracker in devnet.
// Version 9:  Disable smart ancestor selection for the testnet.
//             Enable zstd compression for consensus tonic network in mainnet.
//             Enable passkey auth in multisig for devnet.
//             Remove the iota-bridge from the framework.
// Version 10: Enable min_free_execution_slot for the shared object congestion
//             tracker in all networks.
//             Increase the committee size to 80 on all networks.
//             Enable round prober in consensus for mainnet.
//             Enable probing for accepted rounds in round prober for mainnet.
//             Switch to distributed vote scoring in consensus for mainnet.
//             Enable the new consensus commit rule for mainnet.
//             Enable consensus garbage collection for mainnet with GC depth set
//             to 60 rounds.
//             Enable batching in synchronizer for testnet
//             Enable the gas price feedback mechanism in devnet.
//             Enable Identifier input validation.
//             Removes unnecessary child object mutations
//             Add additional signature checks
//             Add additional linkage checks
// Version 11: Framework fix regarding candidate validator commission rate.
// Version 12: Enable the gas price feedback mechanism in all networks.
//             Enable the normalization of PTB arguments.
// Version 13: Introduce logic to allow the committee to be selected from a set
//             of eligible active validators.
//             Enable processing and tracking AuthorityCapabilitiesV1 from
//             non-committee validators in the devnet.
// Version 14: Switches the consensus protocol to Starfish in devnet.
//             Enable median-based commit timestamp calculation in consensus,
//             and enforce checkpoint timestamp monotonicity for testnet.
//             Enable batched block sync for mainnet.
//             Enable selecting committee only from active validators that
//             support the next epoch's version and issued valid
//             AuthorityCapabilities notification in testnet.
// Version 15: Enable shared object transaction bursts of 10 times average load
//             on devnet.
// Version 16: Enable selecting committee only from active validators that
//             support the next epoch's version and issued valid
//             AuthorityCapabilities notification.
//             Enable committing transactions only for traversed headers in
//             Starfish.
// Version 17: Increase the committee size to 100 on all networks.
// Version 18: Enable passkey authentication support in testnet.
// Version 19: Enable congestion limit overshoot in the gas price feedback
//             mechanism on devnet.
//             Enable a separate gas price feedback mechanism for transactions
//             using randomness on devnet.
//             Allow metadata bytes indexed with a dedicated key in compiled
//             Move modules in devnet.
//             Enable publishing package metadata v1 along with the package in
//             devnet.
//             Enable Move-based account authentication in devnet.
//             Increase the base cost for transfer receive object in devnet.
//             Switch consensus protocol to Starfish in testnet.
//             Enable passkey authentication support in mainnet.
//             Change epoch transaction will contain validator scores.
//             Enable validator scoring on testnet and enable adjustment of
//             validator rewards based on scores on Devnet.
// Version 20: Supports the calculation of validator scores while still passing
//             a default score value to the advance_epoch call. Enables this
//             decoupling on Testnet; Devnet and Mainnet behavior remain the
//             same.
//             Introduce Dynamic Minimum Commission (IIP-8) on all networks.
// Version 21: Enable overshoot of 100 in congestion control on testnet.
//             Enable congestion limit overshoot in the gas price feedback
//             mechanism on testnet.
//             Enable a separate gas price feedback mechanism for transactions
//             using randomness on testnet.
//             Enable fast commit syncer for faster recovery in devnet.
//             Add auth_context_tx native functions costs.
//             Reduce max_auth_gas in Devnet.
// Version 22: Enable overshoot of 100 in congestion control on all networks.
//             Enable congestion limit overshoot in the gas price feedback
//             mechanism on all networks.
//             Enable a separate gas price feedback mechanism for transactions
//             using randomness on all networks.
//             Enable Move-based account authentication in testnet.
//             Enable fast commit syncer for faster recovery on testnet.
// Version 23: Enable Move native context (TxContext via native functions) in
//             all networks. TxContext fields are read via native functions
//             instead of being deserialized from a BCS-encoded struct.
//             Enables sponsor, rgp, gas_price, and gas_budget to be exposed to
//             Move.
// Version 24: Switch consensus protocol to Starfish in all networks.
//             Enable Move-based sponsor account authentication in devnet.
//             Add AuthContext native functions cost for reading tx_data_bytes.
//             Enable additional borrow checks.
// Version 25: Deprecate zkLogin related parameters since zkLogin is no longer
//             supported.
// Version 26: Introduce a module to allow Move code to query protocol feature
//             flags at runtime.
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtocolVersion(u64);

impl ProtocolVersion {
    // The minimum and maximum protocol version supported by this binary.
    // Counterintuitively, this constant may change over time as support for old
    // protocol versions is removed from the source. This ensures that when a
    // new network (such as a testnet) is created, its genesis committee will
    // use a protocol version that is actually supported by the binary.
    pub const MIN: Self = Self(MIN_PROTOCOL_VERSION);

    pub const MAX: Self = Self(MAX_PROTOCOL_VERSION);

    // We create one additional "fake" version in simulator builds so that we can
    // test upgrades.
    #[cfg(msim)]
    pub const MAX_ALLOWED: Self = Self(MAX_PROTOCOL_VERSION + 1);

    pub fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    // For serde deserialization - we don't define a Default impl because there
    // isn't a single universally appropriate default value.
    pub fn max() -> Self {
        Self::MAX
    }
}

impl From<u64> for ProtocolVersion {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

// -----------------------------------------------------------------------------------------
// End of originally contained in crates/iota-protocol-config/src/lib.rs section
// -----------------------------------------------------------------------------------------

/// Use with serde_as to control serde for human-readable serialization and
/// deserialization `H` : serde_as SerializeAs/DeserializeAs delegation for
/// human readable in/output `R` : serde_as SerializeAs/DeserializeAs delegation
/// for non-human readable in/output
///
/// # Example:
///
/// ```text
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Example(#[serde_as(as = "Readable<DisplayFromStr, _>")] [u8; 20]);
/// ```
///
/// The above example will delegate human-readable serde to `DisplayFromStr`
/// and array tuple (default) for non-human-readable serializer.
pub struct Readable<H, R> {
    human_readable: PhantomData<H>,
    non_human_readable: PhantomData<R>,
}

impl<T: ?Sized, H, R> SerializeAs<T> for Readable<H, R>
    where
        H: SerializeAs<T>,
        R: SerializeAs<T>,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if serializer.is_human_readable() {
            H::serialize_as(value, serializer)
        } else {
            R::serialize_as(value, serializer)
        }
    }
}

impl<'de, R, H, T> DeserializeAs<'de, T> for Readable<H, R>
    where
        H: DeserializeAs<'de, T>,
        R: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            H::deserialize_as(deserializer)
        } else {
            R::deserialize_as(deserializer)
        }
    }
}

pub struct IotaStructTag;

impl SerializeAs<StructTag> for IotaStructTag {
    fn serialize_as<S>(value: &StructTag, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let f = to_iota_struct_tag_string(value).map_err(S::Error::custom)?;
        f.serialize(serializer)
    }
}

const IOTA_ADDRESSES: [IotaAddress; 7] = [
    IotaAddress::ZERO,
    IotaAddress::STD,
    IotaAddress::FRAMEWORK,
    IotaAddress::SYSTEM,
    IotaAddress::STARDUST,
    IotaAddress::SYSTEM_STATE,
    IotaAddress::CLOCK,
];
/// Serialize StructTag as a string, retaining the leading zeros in the address.
pub fn to_iota_struct_tag_string(value: &StructTag) -> Result<String, fmt::Error> {
    let mut f = String::new();
    let address = value.address();
    // trim leading zeros if address is in IOTA_ADDRESSES
    let address_str = if IOTA_ADDRESSES.contains(&address) {
        address.to_short_hex()
    } else {
        address.to_canonical_string(/* with_prefix */ true)
    };

    write!(f, "{}::{}::{}", address_str, value.module(), value.name())?;
    if let Some(first_ty) = value.type_params().first() {
        write!(f, "<")?;
        write!(f, "{}", to_iota_type_tag_string(first_ty)?)?;
        for ty in value.type_params().iter().skip(1) {
            write!(f, ", {}", to_iota_type_tag_string(ty)?)?;
        }
        write!(f, ">")?;
    }
    Ok(f)
}

fn to_iota_type_tag_string(value: &TypeTag) -> Result<String, fmt::Error> {
    match value {
        TypeTag::Vector(t) => Ok(format!("vector<{}>", to_iota_type_tag_string(t)?)),
        TypeTag::Struct(s) => to_iota_struct_tag_string(s),
        _ => Ok(value.to_string()),
    }
}

impl<'de> DeserializeAs<'de, StructTag> for IotaStructTag {
    fn deserialize_as<D>(deserializer: D) -> Result<StructTag, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_struct_tag(&s).map_err(D::Error::custom)
    }
}

pub struct IotaTypeTag;

impl SerializeAs<TypeTag> for IotaTypeTag {
    fn serialize_as<S>(value: &TypeTag, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = to_iota_type_tag_string(value).map_err(S::Error::custom)?;
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, TypeTag> for IotaTypeTag {
    fn deserialize_as<D>(deserializer: D) -> Result<TypeTag, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_type_tag(&s).map_err(D::Error::custom)
    }
}

/// A marker for type tags that are serialized as strings. Normally, a
/// type tag is serialized as a string for readable formats, and as a byte array
/// for non-readable formats. This marker can be used to serialize a type tag as
/// a string even in non-readable formats.
pub struct TypeName;

impl SerializeAs<TypeTag> for TypeName {
    fn serialize_as<S>(value: &TypeTag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_canonical_string(false);
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, TypeTag> for TypeName {
    fn deserialize_as<D>(deserializer: D) -> Result<TypeTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_type_tag(&s).map_err(D::Error::custom)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct BigInt<T>(#[serde_as(as = "DisplayFromStr")] T)
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display;

impl<T> BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> SerializeAs<T> for BigInt<T>
    where
        T: Display + FromStr + Copy,
        <T as FromStr>::Err: Display,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        BigInt(*value).serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for BigInt<T>
    where
        T: Display + FromStr + Copy,
        <T as FromStr>::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(*BigInt::deserialize(deserializer)?)
    }
}

impl<T> From<T> for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    fn from(v: T) -> BigInt<T> {
        BigInt(v)
    }
}

impl<T> Deref for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct SequenceNumber(u64);

impl SerializeAs<super::base_types::SequenceNumber> for SequenceNumber {
    fn serialize_as<S>(
        value: &super::base_types::SequenceNumber,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = value.to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, super::base_types::SequenceNumber> for SequenceNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<super::base_types::SequenceNumber, D::Error>
        where
            D: Deserializer<'de>,
    {
        let b = BigInt::deserialize(deserializer)?;
        Ok(super::base_types::SequenceNumber::from_u64(*b))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename = "ProtocolVersion")]
pub struct AsProtocolVersion(u64);

impl SerializeAs<ProtocolVersion> for AsProtocolVersion {
    fn serialize_as<S>(value: &ProtocolVersion, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = value.as_u64().to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, ProtocolVersion> for AsProtocolVersion {
    fn deserialize_as<D>(deserializer: D) -> Result<ProtocolVersion, D::Error>
        where
            D: Deserializer<'de>,
    {
        let b = BigInt::<u64>::deserialize(deserializer)?;
        Ok(ProtocolVersion::from(*b))
    }
}

