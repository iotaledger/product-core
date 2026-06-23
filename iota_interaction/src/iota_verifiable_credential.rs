// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk_types::{ObjectId, TypeTag};
use serde::{Deserialize, Serialize};

use crate::{MoveType, TypedValue};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaVerifiableCredential {
  data: Vec<u8>,
}

impl IotaVerifiableCredential {
  pub fn new(data: Vec<u8>) -> IotaVerifiableCredential {
    IotaVerifiableCredential { data }
  }

  pub fn data(&self) -> &Vec<u8> {
    &self.data
  }
}

impl MoveType for IotaVerifiableCredential {
  fn move_type(package: ObjectId) -> TypeTag {
    TypeTag::from_str(&format!("{package}::public_vc::PublicVc")).expect("valid utf8")
  }

  fn get_typed_value(&self, _package: ObjectId) -> TypedValue<'_, Self>
  where
    Self: MoveType,
    Self: Sized,
  {
    TypedValue::IotaVerifiableCredential(self)
  }
}
