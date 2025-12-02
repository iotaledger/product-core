// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_interaction::rpc_types::IotaObjectDataOptions;
use iota_interaction::types::base_types::{ObjectID, ObjectRef};
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;

use iota_interaction::types::transaction::{Argument, ObjectArg, ProgrammableTransaction};
use iota_interaction::types::{IOTA_CLOCK_OBJECT_ID, IOTA_CLOCK_OBJECT_SHARED_VERSION, TypeTag, Identifier};
use iota_interaction::{ident_str, IotaClientTrait, OptionalSync};
use crate::core_client::CoreClientReadOnly;
use serde::Serialize;

use crate::error::Error;

/// Adds a reference to the on-chain clock to `ptb`'s arguments.
pub fn get_clock_ref(ptb: &mut Ptb) -> Argument {
    ptb.obj(ObjectArg::SharedObject {
        id: IOTA_CLOCK_OBJECT_ID,
        initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
        mutable: false,
    })
    .expect("network has a singleton clock instantiated")
}

pub fn ptb_pure<T>(ptb: &mut Ptb, name: &str, value: T) -> Result<Argument, Error>
where
    T: Serialize + core::fmt::Debug,
{
    ptb.pure(&value).map_err(|err| {
        Error::InvalidMoveArgument(format!(
            r"could not serialize pure value {name} with value {value:?}; {err}"
        ))
    })
}

/// Get the type tag of an object
pub async fn get_type_tag<C>(client: &C, object_id: &ObjectID) -> Result<TypeTag, Error>
where
    C: CoreClientReadOnly + OptionalSync,
{
    let object_response = client
        .client_adapter()
        .read_api()
        .get_object_with_options(*object_id, IotaObjectDataOptions::new().with_type())
        .await
        .map_err(|err| Error::FailedToParseMoveTag(format!("Failed to get object: {err}")))?;

    let object_data = object_response
        .data
        .ok_or_else(|| Error::FailedToParseMoveTag(format!("Object {object_id} not found")))?;

    let full_type_str = object_data
        .object_type()
        .map_err(|e| Error::FailedToParseMoveTag(format!("Failed to get object type: {e}")))?
        .to_string();

    let type_param_str = parse_type(&full_type_str)?;

    let tag = TypeTag::from_str(&type_param_str)
        .map_err(|e| Error::FailedToParseMoveTag(format!("Failed to parse tag '{type_param_str}': {e}")))?;

    Ok(tag)
}

/// Parses the type string to get the generic argument
///
/// # Example
///
/// ```rust,ignore
/// let full_type = "0x123::notarization::Notarization<vector<u8>>";
/// let type_param_str = parse_type(full_type).unwrap();
/// assert_eq!(type_param_str, "vector<u8>");
/// ```
pub fn parse_type(full_type: &str) -> Result<String, Error> {
    if let (Some(start), Some(end)) = (full_type.find('<'), full_type.rfind('>')) {
        Ok(full_type[start + 1..end].to_string())
    } else {
        Err(Error::FailedToParseMoveTag(format!(
            "Could not parse type parameter from {full_type}"
        )))
    }
}

pub async fn get_object_ref_by_id(
    iota_client: &impl CoreClientReadOnly,
    obj: &ObjectID,
) -> Result<ObjectRef, Error> {
    let res = iota_client
        .client_adapter()
        .read_api()
        .get_object_with_options(*obj, IotaObjectDataOptions::new().with_content())
        .await
        .map_err(|err| Error::FailedToGetMoveObject(err.to_string()))?;

    let Some(data) = res.data else {
        return Err(Error::InvalidMoveArgument("no data found".to_string()));
    };

    Ok(data.object_ref())
}

/// Helper to create a new transaction and run a closure that injects the
/// some logic.
///
/// # Arguments
/// * `client` - A CoreClientReadOnly implementation
/// * `contract_name` - The contract name to call
/// * `object_id` - object ID for the notarization
/// * `method` - The method name to call
/// * `additional_args` - Closure providing additional arguments for the transaction
///
/// # Type Parameters
/// * `C` - CoreClientReadOnly implementation
/// * `F` - Closure type that produces additional arguments
///
/// # Errors
/// Returns `Error` if:
/// * Tag retrieval fails
/// * Object reference retrieval fails
/// * Transaction building fails
/// * Method name is invalid
pub async fn build_transaction<C, F>(
    client: &C,
    contract_name: &'static str,
    object_id: ObjectID,
    method: impl AsRef<str>,
    additional_args: F,
) -> Result<ProgrammableTransaction, Error>
where
  F: FnOnce(&mut Ptb) -> Result<Vec<Argument>, Error>,
  C: CoreClientReadOnly + OptionalSync,
{
    let mut ptb = Ptb::new();

    let tag = vec![get_type_tag(client, &object_id).await?];

    let mut args = {
        let notarization = get_object_ref_by_id(client, &object_id).await?;

        vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(notarization))
              .map_err(|e| Error::InvalidMoveArgument(format!("Failed to create object argument: {e}")))?,
        ]
    };
    // Add additional arguments
    args.extend(
        additional_args(&mut ptb)
          .map_err(|e| Error::InvalidMoveArgument(format!("Failed to add additional arguments: {e}")))?,
    );

    // Create method identifier
    let function = Identifier::from_str(method.as_ref())
      .map_err(|e| Error::InvalidMoveArgument(format!("Invalid method name '{}': {}", method.as_ref(), e)))?;

    // Build the move call
    ptb.programmable_move_call(
        client.package_id(),
        ident_str!(contract_name).into(),
        function,
        tag,
        args,
    );

    Ok(ptb.finish())
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
