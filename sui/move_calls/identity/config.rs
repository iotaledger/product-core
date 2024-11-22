// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::str::FromStr;

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use move_core_types::ident_str;

use super::super::utils;

#[allow(clippy::too_many_arguments)]
pub(crate) fn propose_config_change<I1, I2>(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  expiration: Option<u64>,
  threshold: Option<u64>,
  controllers_to_add: I1,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_to_update: I2,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction>
where
  I1: IntoIterator<Item = (IotaAddress, u64)>,
  I2: IntoIterator<Item = (ObjectID, u64)>,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let controllers_to_add = {
    let (addresses, vps): (Vec<IotaAddress>, Vec<u64>) = controllers_to_add.into_iter().unzip();
    let addresses = ptb.pure(addresses)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      ident_str!("utils").into(),
      ident_str!("vec_map_from_keys_values").into(),
      vec![TypeTag::Address, TypeTag::U64],
      vec![addresses, vps],
    )
  };
  let controllers_to_update = {
    let (ids, vps): (Vec<ObjectID>, Vec<u64>) = controllers_to_update.into_iter().unzip();
    let ids = ptb.pure(ids)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      ident_str!("utils").into(),
      ident_str!("vec_map_from_keys_values").into(),
      vec![TypeTag::from_str("0x2::object::ID").expect("valid utf8"), TypeTag::U64],
      vec![ids, vps],
    )
  };
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let expiration = utils::option_to_move(expiration, &mut ptb, package)?;
  let threshold = utils::option_to_move(threshold, &mut ptb, package)?;
  let controllers_to_remove = ptb.pure(controllers_to_remove)?;

  let _proposal_id = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("propose_config_change").into(),
    vec![],
    vec![
      identity,
      controller_cap,
      expiration,
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    ],
  );

  Ok(ptb.finish())
}

pub(crate) fn execute_config_change(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  proposal_id: ObjectID,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let mut ptb = ProgrammableTransactionBuilder::new();

  let Owner::Shared { initial_shared_version } = identity.owner else {
    anyhow::bail!("identity \"{}\" is a not shared object", identity.reference.object_id);
  };
  let identity = ptb.obj(ObjectArg::SharedObject {
    id: identity.reference.object_id,
    initial_shared_version,
    mutable: true,
  })?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let proposal_id = ptb.pure(proposal_id)?;
  ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_config_change").into(),
    vec![],
    vec![identity, controller_cap, proposal_id],
  );

  Ok(ptb.finish())
}
