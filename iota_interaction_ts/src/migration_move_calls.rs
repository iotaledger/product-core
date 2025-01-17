// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::ident_str;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use identity_iota_interaction::MigrationMoveCalls;
use identity_iota_interaction::ProgrammableTransactionBcs;

use crate::error::TsSdkError;

pub struct MigrationMoveCallsTsSdk {}

impl MigrationMoveCalls for MigrationMoveCallsTsSdk {
  type Error = TsSdkError;

  fn migrate_did_output(
    did_output: ObjectRef,
    creation_timestamp: Option<u64>,
    migration_registry: OwnedObjectRef,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }
}
