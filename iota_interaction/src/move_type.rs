// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_types::{ObjectId, TypeTag};
use serde::Serialize;

use crate::types::base_types::IotaAddress;
use crate::IotaVerifiableCredential;

pub enum TypedValue<'a, T: MoveType> {
  IotaVerifiableCredential(&'a IotaVerifiableCredential),
  Other(&'a T),
}

/// Trait for types that can be converted to a Move type.
pub trait MoveType<T: Serialize = Self>: Serialize {
  /// Returns the Move type for this type.
  fn move_type(package: ObjectId) -> TypeTag;

  fn get_typed_value(&self, _package: ObjectId) -> TypedValue<'_, Self>
  where
    Self: MoveType,
    Self: Sized,
  {
    TypedValue::Other(self)
  }
}

impl MoveType for u8 {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::U8
  }
}

impl MoveType for u16 {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::U16
  }
}

impl MoveType for u32 {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::U32
  }
}

impl MoveType for u64 {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::U64
  }
}

impl MoveType for u128 {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::U128
  }
}

impl MoveType for bool {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::Bool
  }
}

impl MoveType for IotaAddress {
  fn move_type(_package: ObjectId) -> TypeTag {
    TypeTag::Address
  }
}

impl<T: MoveType> MoveType for Vec<T> {
  fn move_type(package: ObjectId) -> TypeTag {
    TypeTag::Vector(Box::new(T::move_type(package)))
  }
}
