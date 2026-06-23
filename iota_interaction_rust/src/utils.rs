// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use iota_interaction::interaction_error::Error;
use iota_interaction::rpc_types::OwnedObjectRef;
use iota_interaction::types::base_types::RESOLVED_STD_OPTION;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_interaction::types::transaction::{CallArg, SharedObjectRef};
use iota_interaction::types::{IOTA_CLOCK_OBJECT_ID, IOTA_CLOCK_OBJECT_SHARED_VERSION, MOVE_STDLIB_PACKAGE_ID};
use iota_interaction::MoveType;
use iota_sdk_types::{Argument, Identifier, ObjectId, Owner};
use serde::Serialize;

/// Adds a reference to the on-chain clock to `ptb`'s arguments.
pub fn get_clock_ref(ptb: &mut Ptb) -> Argument {
  ptb
    .obj(CallArg::Shared(SharedObjectRef {
      object_id: IOTA_CLOCK_OBJECT_ID,
      initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
      mutable: false,
    }))
    .expect("network has a singleton clock instantiated")
}

pub fn get_controller_delegation(ptb: &mut Ptb, controller_cap: Argument, package: ObjectId) -> (Argument, Argument) {
  let Argument::Result(idx) = ptb.programmable_move_call(
    package,
    Identifier::from_static("controller"),
    Identifier::from_static("borrow"),
    vec![],
    vec![controller_cap],
  ) else {
    unreachable!("making move calls always return a result variant");
  };

  (Argument::NestedResult(idx, 0), Argument::NestedResult(idx, 1))
}

pub fn put_back_delegation_token(
  ptb: &mut Ptb,
  controller_cap: Argument,
  delegation_token: Argument,
  borrow: Argument,
  package: ObjectId,
) {
  ptb.programmable_move_call(
    package,
    Identifier::from_static("controller"),
    Identifier::from_static("put_back"),
    vec![],
    vec![controller_cap, delegation_token, borrow],
  );
}

pub fn owned_ref_to_shared_object_arg(
  owned_ref: OwnedObjectRef,
  ptb: &mut Ptb,
  mutable: bool,
) -> anyhow::Result<Argument> {
  let Owner::Shared(initial_shared_version) = owned_ref.owner else {
    anyhow::bail!("Object \"{}\" is not a shared object", owned_ref.object_id());
  };
  ptb.obj(CallArg::Shared(SharedObjectRef {
    object_id: owned_ref.object_id(),
    initial_shared_version,
    mutable,
  }))
}

pub fn option_to_move<T: MoveType + Serialize>(
  option: Option<T>,
  ptb: &mut Ptb,
  package: ObjectId,
) -> Result<Argument, anyhow::Error> {
  let arg = if let Some(t) = option {
    let t = ptb.pure(t)?;
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      Identifier::from_static(RESOLVED_STD_OPTION.1.as_str()),
      Identifier::from_static("some"),
      vec![T::move_type(package)],
      vec![t],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      Identifier::from_static(RESOLVED_STD_OPTION.1.as_str()),
      Identifier::from_static("none"),
      vec![T::move_type(package)],
      vec![],
    )
  };

  Ok(arg)
}

pub fn ptb_pure<T>(ptb: &mut Ptb, name: &str, value: T) -> Result<Argument, Error>
where
  T: Serialize + Debug,
{
  ptb.pure(&value).map_err(|err| {
    Error::InvalidArgument(format!(
      r"could not serialize pure value {name} with value {value:?}; {err}"
    ))
  })
}

#[allow(dead_code)]
pub fn ptb_obj(ptb: &mut Ptb, name: &str, value: impl Into<CallArg> + Debug + Copy) -> Result<Argument, Error> {
  ptb
    .obj(value)
    .map_err(|err| Error::InvalidArgument(format!("could not serialize object {name} {value:?}; {err}")))
}
