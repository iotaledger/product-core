// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_interaction::rpc_types::IotaObjectDataOptions;
use iota_interaction::types::base_types::{ObjectID, STD_OPTION_MODULE_NAME, STD_UTF8_MODULE_NAME};
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_interaction::types::transaction::{Argument, ObjectArg};
use iota_interaction::types::{
  TypeTag, IOTA_CLOCK_OBJECT_ID, IOTA_CLOCK_OBJECT_SHARED_VERSION, MOVE_STDLIB_PACKAGE_ID,
};
use iota_interaction::{ident_str, IotaClientTrait, MoveType, OptionalSync};
use serde::Serialize;

use crate::core_client::CoreClientReadOnly;
use crate::error::Error;

/// Adds a reference to the on-chain clock to `ptb`'s arguments.
pub fn get_clock_ref(ptb: &mut Ptb) -> Argument {
  ptb
    .obj(ObjectArg::SharedObject {
      id: IOTA_CLOCK_OBJECT_ID,
      initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
      mutable: false,
    })
    .expect("network has a singleton clock instantiated")
}

pub fn option_to_move_with_tag(option: Option<Argument>, tag: TypeTag, ptb: &mut Ptb) -> Result<Argument, Error> {
  let arg = if let Some(t) = option {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("some").into(),
      vec![tag],
      vec![t],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("none").into(),
      vec![tag],
      vec![],
    )
  };

  Ok(arg)
}

pub fn ptb_pure<T>(ptb: &mut Ptb, name: &str, value: T) -> Result<Argument, Error>
where
  T: Serialize + core::fmt::Debug,
{
  ptb.pure(&value).map_err(|err| {
    Error::InvalidArgument(format!(
      r"could not serialize pure value {name} with value {value:?}; {err}"
    ))
  })
}

#[allow(dead_code)]
pub fn ptb_obj(ptb: &mut Ptb, name: &str, value: ObjectArg) -> Result<Argument, Error> {
  ptb
    .obj(value)
    .map_err(|err| Error::InvalidArgument(format!("could not serialize object {name} {value:?}; {err}")))
}

pub fn option_to_move<T: MoveType + Serialize>(
  option: Option<T>,
  ptb: &mut Ptb,
  package: ObjectID,
) -> Result<Argument, Error> {
  let arg = if let Some(t) = option {
    let t = ptb
      .pure(t)
      .map_err(|err| Error::InvalidArgument(format!("could not serialize pure value; {err}")))?;
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("some").into(),
      vec![T::move_type(package)],
      vec![t],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("none").into(),
      vec![T::move_type(package)],
      vec![],
    )
  };

  Ok(arg)
}

/// Creates a new move string
pub fn new_move_string(value: String, ptb: &mut Ptb) -> Result<Argument, Error> {
  let v = ptb
    .pure(value.as_bytes())
    .map_err(|err| Error::InvalidArgument(format!("could not serialize string value; {err}")))?;
  Ok(ptb.programmable_move_call(
    MOVE_STDLIB_PACKAGE_ID,
    STD_UTF8_MODULE_NAME.into(),
    ident_str!("utf8").into(),
    vec![],
    vec![v],
  ))
}

/// Create new option string
pub fn new_move_option_string(value: Option<String>, ptb: &mut Ptb) -> Result<Argument, Error> {
  let string_tag = TypeTag::from_str(format!("{}::string::String", MOVE_STDLIB_PACKAGE_ID).as_str())
    .map_err(|err| Error::InvalidArgument(format!("could not create string tag; {err}")))?;

  match value {
    Some(v) => {
      let v = ptb
        .pure(v.as_bytes())
        .map_err(|err| Error::InvalidArgument(format!("could not serialize string value; {err}")))?;
      Ok(ptb.programmable_move_call(
        MOVE_STDLIB_PACKAGE_ID,
        STD_OPTION_MODULE_NAME.into(),
        ident_str!("some").into(),
        vec![string_tag],
        vec![v],
      ))
    }
    None => Ok(ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("none").into(),
      vec![string_tag],
      vec![],
    )),
  }
}

pub async fn get_type_tag<C>(client: &C, object_id: &ObjectID) -> Result<TypeTag, Error>
where
  C: CoreClientReadOnly + OptionalSync,
{
  let object_response = client
    .client_adapter()
    .read_api()
    .get_object_with_options(*object_id, IotaObjectDataOptions::new().with_type())
    .await
    .map_err(|err| Error::FailedToParseTag(format!("Failed to get object: {err}")))?;

  let object_data = object_response
    .data
    .ok_or_else(|| Error::FailedToParseTag(format!("Object {} not found", object_id)))?;

  let full_type_str = object_data
    .object_type()
    .map_err(|e| Error::FailedToParseTag(format!("Failed to get object type: {e}")))?
    .to_string();

  let type_param_str = parse_type(&full_type_str)?;

  let tag = TypeTag::from_str(&type_param_str)
    .map_err(|e| Error::FailedToParseTag(format!("Failed to parse tag '{}': {}", type_param_str, e)))?;

  Ok(tag)
}

/// Parses the type string to get the generic argument
///
/// # Example
///
/// ```no_run
/// let full_type = "0x123::notarization::Notarization<vector<u8>>";
/// let type_param_str = parse_type(full_type).unwrap();
/// assert_eq!(type_param_str, "vector<u8>");
/// ```
pub fn parse_type(full_type: &str) -> Result<String, Error> {
  if let (Some(start), Some(end)) = (full_type.find('<'), full_type.rfind('>')) {
    Ok(full_type[start + 1..end].to_string())
  } else {
    Err(Error::FailedToParseTag(format!(
      "Could not parse type parameter from {}",
      full_type
    )))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_type() {
    let full_type = "0x123::notarization::Notarization<vector<u8>>";
    let type_param_str = parse_type(full_type).unwrap();
    assert_eq!(type_param_str, "vector<u8>");
  }

  #[test]
  fn test_parse_type_invalid() {
    let full_type = "0x123::notarization::Notarization";
    let type_param_str = parse_type(full_type);
    assert!(type_param_str.is_err());
  }

  #[test]
  fn test_parse_type_nested_generics() {
    let full_type = "0x123::notarization::Complex<Option<vector<u8>>>";
    let type_param_str = parse_type(full_type).unwrap();
    assert_eq!(type_param_str, "Option<vector<u8>>");
  }

  #[test]
  fn test_parse_type_multiple_generics() {
    let full_type = "0x123::notarization::Pair<u8, vector<u8>>";
    let type_param_str = parse_type(full_type).unwrap();
    assert_eq!(type_param_str, "u8, vector<u8>");
  }

  #[test]
  fn test_parse_type_empty_generics() {
    let full_type = "0x123::notarization::Empty<>";
    let type_param_str = parse_type(full_type).unwrap();
    assert_eq!(type_param_str, "");
  }
}
