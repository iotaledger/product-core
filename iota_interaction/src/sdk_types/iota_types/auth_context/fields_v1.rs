// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::ident_str;
use crate::move_core_types::{identifier::IdentStr, language_storage::StructTag};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::{
    IOTA_FRAMEWORK_ADDRESS,
    base_types::{ObjectID, ObjectRef, SequenceNumber, TypeTag},
    iota_serde::TypeName,
    transaction::{Argument, CallArg, Command},
};

// ---------------------------------------------------------------------------
// Module / struct name constants
// ---------------------------------------------------------------------------

pub const CALL_ARG_MODULE_NAME: &IdentStr = ident_str!("ptb_call_arg");
pub const CALL_ARG_STRUCT_NAME: &IdentStr = ident_str!("CallArg");
pub const OBJECT_ARG_STRUCT_NAME: &IdentStr = ident_str!("ObjectArg");
pub const OBJECT_REF_STRUCT_NAME: &IdentStr = ident_str!("ObjectRef");

pub const COMMAND_MODULE_NAME: &IdentStr = ident_str!("ptb_command");
pub const COMMAND_STRUCT_NAME: &IdentStr = ident_str!("Command");
pub const ARGUMENT_STRUCT_NAME: &IdentStr = ident_str!("Argument");
pub const PROGRAMMABLE_MOVE_CALL_STRUCT_NAME: &IdentStr = ident_str!("ProgrammableMoveCall");
pub const TRANSFER_OBJECTS_DATA_STRUCT_NAME: &IdentStr = ident_str!("TransferObjectsData");
pub const SPLIT_COINS_DATA_STRUCT_NAME: &IdentStr = ident_str!("SplitCoinsData");
pub const MERGE_COINS_DATA_STRUCT_NAME: &IdentStr = ident_str!("MergeCoinsData");
pub const PUBLISH_DATA_STRUCT_NAME: &IdentStr = ident_str!("PublishData");
pub const MAKE_MOVE_VEC_DATA_STRUCT_NAME: &IdentStr = ident_str!("MakeMoveVecData");
pub const UPGRADE_DATA_STRUCT_NAME: &IdentStr = ident_str!("UpgradeData");

// ---------------------------------------------------------------------------
// MoveProgrammableMoveCall
// ---------------------------------------------------------------------------

/// Mirrors [`crate::transaction::ProgrammableMoveCall`] for use in
/// [`MoveCommand`], substituting [`TypeTag`] for a string in the type arguments
/// so that the type matches the BCS layout expected by the Move-side
/// `ptb_command::ProgrammableMoveCall`.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveProgrammableMoveCall {
    pub package: ObjectID,
    pub module: String,
    pub function: String,
    #[serde_as(as = "Vec<TypeName>")]
    pub type_arguments: Vec<TypeTag>,
    pub arguments: Vec<Argument>,
}

// ---------------------------------------------------------------------------
// MoveCommand
// ---------------------------------------------------------------------------

/// Mirrors [`crate::transaction::Command`], substituting [`TypeTag`] for
/// a string in `MoveCall` and `MakeMoveVec` so that
/// the type matches the BCS layout expected by the Move-side
/// `ptb_command::Command`.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveCommand {
    MoveCall(Box<MoveProgrammableMoveCall>),
    TransferObjects(Vec<Argument>, Argument),
    SplitCoins(Argument, Vec<Argument>),
    MergeCoins(Argument, Vec<Argument>),
    Publish(Vec<Vec<u8>>, Vec<ObjectID>),
    MakeMoveVec(
        #[serde_as(as = "Option<TypeName>")] Option<TypeTag>,
        Vec<Argument>,
    ),
    Upgrade(Vec<Vec<u8>>, Vec<ObjectID>, ObjectID, Argument),
}

impl From<&Command> for MoveCommand {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::MoveCall(cmd) => MoveCommand::MoveCall(Box::new(MoveProgrammableMoveCall {
                package: cmd.package,
                module: cmd.module.to_string(),
                function: cmd.function.to_string(),
                type_arguments: cmd.type_arguments.clone(),
                arguments: cmd.arguments.clone(),
            })),
            Command::TransferObjects(cmd) => {
                MoveCommand::TransferObjects(cmd.objects.clone(), cmd.address)
            }
            Command::SplitCoins(cmd) => MoveCommand::SplitCoins(cmd.coin, cmd.amounts.clone()),
            Command::MergeCoins(cmd) => {
                MoveCommand::MergeCoins(cmd.coin, cmd.coins_to_merge.clone())
            }
            Command::Publish(cmd) => {
                MoveCommand::Publish(cmd.modules.clone(), cmd.dependencies.clone())
            }
            Command::MakeMoveVector(cmd) => {
                MoveCommand::MakeMoveVec(cmd.type_.clone(), cmd.elements.clone())
            }
            Command::Upgrade(cmd) => MoveCommand::Upgrade(
                cmd.modules.clone(),
                cmd.dependencies.clone(),
                cmd.package,
                cmd.ticket,
            ),
            _ => unimplemented!("a new Command enum variant was added and needs to be handled"),
        }
    }
}

impl MoveCommand {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: COMMAND_MODULE_NAME.to_owned(),
            name: COMMAND_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

// ---------------------------------------------------------------------------
// MoveCallArg
// ---------------------------------------------------------------------------

/// Mirrors `ObjectArg`, matching the BCS layout expected
/// by the Move-side `ptb_call_arg::ObjectArg`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveObjectArg {
    ImmOrOwnedObject(ObjectRef),
    SharedObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    },
    Receiving(ObjectRef),
}

impl MoveObjectArg {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: CALL_ARG_MODULE_NAME.to_owned(),
            name: OBJECT_ARG_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Mirrors [`crate::transaction::CallArg`], matching the BCS layout expected
/// by the Move-side `ptb_call_arg::CallArg`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveCallArg {
    Pure(Vec<u8>),
    Object(MoveObjectArg),
}

impl From<&CallArg> for MoveCallArg {
    fn from(arg: &CallArg) -> Self {
        match arg {
            CallArg::Pure(bytes) => MoveCallArg::Pure(bytes.clone()),
            CallArg::ImmutableOrOwned(obj_arg) => {
                MoveCallArg::Object(MoveObjectArg::ImmOrOwnedObject(*obj_arg))
            }
            CallArg::Shared(obj_arg) => MoveCallArg::Object(MoveObjectArg::SharedObject {
                id: obj_arg.object_id,
                initial_shared_version: obj_arg.initial_shared_version,
                mutable: obj_arg.mutable,
            }),
            CallArg::Receiving(obj_arg) => MoveCallArg::Object(MoveObjectArg::Receiving(*obj_arg)),
            _ => unimplemented!("a new CallArg enum variant was added and needs to be handled"),
        }
    }
}

impl MoveCallArg {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: CALL_ARG_MODULE_NAME.to_owned(),
            name: CALL_ARG_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }
}