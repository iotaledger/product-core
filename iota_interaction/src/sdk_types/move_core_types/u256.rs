// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::mem::size_of;
use std::ops::{Shl, Shr};

// This U256 impl was chosen for now but we are open to changing it as needed
use primitive_types::U256 as PrimitiveU256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uint::FromStrRadixErr;

const NUM_BITS_PER_BYTE: usize = 8;
const U256_NUM_BITS: usize = 256;
pub const U256_NUM_BYTES: usize = U256_NUM_BITS / NUM_BITS_PER_BYTE;

#[derive(Debug)]
pub struct U256FromStrError(FromStrRadixErr);

/// A list of error categories encountered when parsing numbers.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum U256CastErrorKind {
    /// Value too large to fit in U8.
    TooLargeForU8,

    /// Value too large to fit in U16.
    TooLargeForU16,

    /// Value too large to fit in U32.
    TooLargeForU32,

    /// Value too large to fit in U64.
    TooLargeForU64,

    /// Value too large to fit in U128.
    TooLargeForU128,
}

#[derive(Debug)]
pub struct U256CastError {
    kind: U256CastErrorKind,
    val: U256,
}

impl U256CastError {
    pub fn new<T: std::convert::Into<U256>>(val: T, kind: U256CastErrorKind) -> Self {
        Self {
            kind,
            val: val.into(),
        }
    }
}

impl std::error::Error for U256CastError {}

impl fmt::Display for U256CastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self.kind {
            U256CastErrorKind::TooLargeForU8 => "u8",
            U256CastErrorKind::TooLargeForU16 => "u16",
            U256CastErrorKind::TooLargeForU32 => "u32",
            U256CastErrorKind::TooLargeForU64 => "u64",
            U256CastErrorKind::TooLargeForU128 => "u128",
        };
        let err_str = format!("Cast failed. {} too large for {}.", self.val, type_str);
        write!(f, "{err_str}")
    }
}

impl std::error::Error for U256FromStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl fmt::Display for U256FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, PartialOrd, Ord, Default)]
pub struct U256(PrimitiveU256);

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl fmt::LowerHex for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::str::FromStr for U256 {
    type Err = U256FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10)
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(U256::from_le_bytes(
            &(<[u8; U256_NUM_BYTES]>::deserialize(deserializer)?),
        ))
    }
}

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.to_le_bytes().serialize(serializer)
    }
}

impl U256 {
    /// Zero value as U256
    pub const fn zero() -> Self {
        Self(PrimitiveU256::zero())
    }

    /// One value as U256
    pub const fn one() -> Self {
        Self(PrimitiveU256::one())
    }

    /// Max value of U256:
    /// 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    pub const fn max_value() -> Self {
        Self(PrimitiveU256::max_value())
    }

    /// U256 from string with radix 10 or 16
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, U256FromStrError> {
        PrimitiveU256::from_str_radix(src.trim_start_matches('0'), radix)
            .map(Self)
            .map_err(U256FromStrError)
    }

    /// U256 from 32 little endian bytes
    pub fn from_le_bytes(slice: &[u8; U256_NUM_BYTES]) -> Self {
        Self(PrimitiveU256::from_little_endian(slice))
    }

    /// U256 from 32 big endian bytes
    pub fn from_be_bytes(slice: &[u8; U256_NUM_BYTES]) -> Self {
        Self(PrimitiveU256::from_big_endian(slice))
    }

    /// U256 to 32 little endian bytes
    pub fn to_le_bytes(self) -> [u8; U256_NUM_BYTES] {
        let mut bytes = [0u8; U256_NUM_BYTES];
        self.0.to_little_endian(&mut bytes);
        bytes
    }

    /// U256 to 32 big endian bytes
    pub fn to_be_bytes(self) -> [u8; U256_NUM_BYTES] {
        let mut bytes = [0u8; U256_NUM_BYTES];
        self.0.to_big_endian(&mut bytes);
        bytes
    }

    /// Leading zeros of the number
    pub fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    // Unchecked downcasting. Values as truncated if larger than target max
    pub fn unchecked_as_u8(&self) -> u8 {
        self.0.low_u128() as u8
    }

    pub fn unchecked_as_u16(&self) -> u16 {
        self.0.low_u128() as u16
    }

    pub fn unchecked_as_u32(&self) -> u32 {
        self.0.low_u128() as u32
    }

    pub fn unchecked_as_u64(&self) -> u64 {
        self.0.low_u128() as u64
    }

    pub fn unchecked_as_u128(&self) -> u128 {
        self.0.low_u128()
    }

    // Check arithmetic
    /// Checked integer addition. Computes self + rhs, returning None if
    /// overflow occurred.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// Checked integer subtraction. Computes self - rhs, returning None if
    /// overflow occurred.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    /// Checked integer multiplication. Computes self * rhs, returning None if
    /// overflow occurred.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.0.checked_mul(rhs.0).map(Self)
    }

    /// Checked integer division. Computes self / rhs, returning None if rhs ==
    /// 0.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        self.0.checked_div(rhs.0).map(Self)
    }

    /// Checked integer remainder. Computes self % rhs, returning None if rhs ==
    /// 0.
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        self.0.checked_rem(rhs.0).map(Self)
    }

    /// Checked integer remainder. Computes self % rhs, returning None if rhs ==
    /// 0.
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= U256_NUM_BITS as u32 {
            return None;
        }
        Some(Self(self.0.shl(rhs)))
    }

    /// Checked shift right. Computes self >> rhs, returning None if rhs is
    /// larger than or equal to the number of bits in self.
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= U256_NUM_BITS as u32 {
            return None;
        }
        Some(Self(self.0.shr(rhs)))
    }

    /// Downcast to an unsigned value of type T
    /// T must be at most u128
    pub fn down_cast_lossy<T: std::convert::TryFrom<u128>>(self) -> T {
        // Size of this type
        let type_size = size_of::<T>();
        // Maximum value for this type
        let max_val: u128 = if type_size < 16 {
            (1u128 << (NUM_BITS_PER_BYTE * type_size)) - 1u128
        } else {
            u128::MAX
        };
        // This should never fail
        match T::try_from(self.0.low_u128() & max_val) {
            Ok(w) => w,
            Err(_) => panic!("Fatal! Downcast failed"),
        }
    }

    /// Wrapping integer addition. Computes self + rhs,  wrapping around at the
    /// boundary of the type. By definition in std::instrinsics,
    /// a.wrapping_add(b) = (a + b) % (2^N), where N is bit width
    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.overflowing_add(rhs.0).0)
    }

    /// Wrapping integer subtraction. Computes self - rhs,  wrapping around at
    /// the boundary of the type. By definition in std::instrinsics,
    /// a.wrapping_add(b) = (a - b) % (2^N), where N is bit width
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.overflowing_sub(rhs.0).0)
    }

    /// Wrapping integer multiplication. Computes self * rhs,  wrapping around
    /// at the boundary of the type. By definition in std::instrinsics,
    /// a.wrapping_mul(b) = (a * b) % (2^N), where N is bit width
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.overflowing_mul(rhs.0).0)
    }
}

impl From<u8> for U256 {
    fn from(n: u8) -> Self {
        U256(PrimitiveU256::from(n))
    }
}

impl From<u16> for U256 {
    fn from(n: u16) -> Self {
        U256(PrimitiveU256::from(n))
    }
}

impl From<u32> for U256 {
    fn from(n: u32) -> Self {
        U256(PrimitiveU256::from(n))
    }
}

impl From<u64> for U256 {
    fn from(n: u64) -> Self {
        U256(PrimitiveU256::from(n))
    }
}

impl From<u128> for U256 {
    fn from(n: u128) -> Self {
        U256(PrimitiveU256::from(n))
    }
}

impl TryFrom<U256> for u8 {
    type Error = U256CastError;
    fn try_from(n: U256) -> Result<Self, Self::Error> {
        let n = n.0.low_u64();
        if n > u8::MAX as u64 {
            Err(U256CastError::new(n, U256CastErrorKind::TooLargeForU8))
        } else {
            Ok(n as u8)
        }
    }
}

impl TryFrom<U256> for u16 {
    type Error = U256CastError;

    fn try_from(n: U256) -> Result<Self, Self::Error> {
        let n = n.0.low_u64();
        if n > u16::MAX as u64 {
            Err(U256CastError::new(n, U256CastErrorKind::TooLargeForU16))
        } else {
            Ok(n as u16)
        }
    }
}

impl TryFrom<U256> for u32 {
    type Error = U256CastError;

    fn try_from(n: U256) -> Result<Self, Self::Error> {
        let n = n.0.low_u64();
        if n > u32::MAX as u64 {
            Err(U256CastError::new(n, U256CastErrorKind::TooLargeForU32))
        } else {
            Ok(n as u32)
        }
    }
}

impl TryFrom<U256> for u64 {
    type Error = U256CastError;

    fn try_from(n: U256) -> Result<Self, Self::Error> {
        let n = n.0.low_u128();
        if n > u64::MAX as u128 {
            Err(U256CastError::new(n, U256CastErrorKind::TooLargeForU64))
        } else {
            Ok(n as u64)
        }
    }
}

impl TryFrom<U256> for u128 {
    type Error = U256CastError;

    fn try_from(n: U256) -> Result<Self, Self::Error> {
        if n > U256::from(u128::MAX) {
            Err(U256CastError::new(n, U256CastErrorKind::TooLargeForU128))
        } else {
            Ok(n.0.low_u128())
        }
    }
}