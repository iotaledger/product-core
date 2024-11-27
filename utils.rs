// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::TypeTag;
use serde::Serialize;
use tokio::process::Command;

use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::IotaClient;
use iota_sdk::IotaClientBuilder;

use crate::rebased::Error;

/// The local `IOTA` network.
pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

/// Builds an `IOTA` client for the given network.
pub async fn get_client(network: &str) -> Result<IotaClient, Error> {
  let client = IotaClientBuilder::default()
    .build(network)
    .await
    .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

  Ok(client)
}

/// Requests funds from the local `IOTA` faucet.
pub async fn request_funds(address: &IotaAddress) -> anyhow::Result<()> {
  let output = Command::new("iota")
    .arg("client")
    .arg("faucet")
    .arg("--address")
    .arg(address.to_string())
    .arg("--url")
    .arg("http://127.0.0.1:9123/gas")
    .arg("--json")
    .output()
    .await
    .context("Failed to execute command")?;

  if !output.status.success() {
    anyhow::bail!(
      "Failed to request funds from faucet: {}",
      std::str::from_utf8(&output.stderr).unwrap()
    );
  }

  Ok(())
}

/// Trait for types that can be converted to a Move type.
pub trait MoveType<T: Serialize = Self>: Serialize {
  /// Returns the Move type for this type.
  fn move_type(package: ObjectID) -> TypeTag;

  /// Tries to convert this type to a Move argument.
  fn try_to_argument(
    &self,
    ptb: &mut ProgrammableTransactionBuilder,
    _package: Option<ObjectID>,
  ) -> Result<Argument, Error> {
    ptb.pure(self).map_err(|e| Error::InvalidArgument(e.to_string()))
  }
}

impl MoveType for u8 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U8
  }
}

impl MoveType for u16 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U16
  }
}

impl MoveType for u32 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U32
  }
}

impl MoveType for u64 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U64
  }
}

impl MoveType for u128 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U128
  }
}

impl MoveType for bool {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::Bool
  }
}

impl MoveType for IotaAddress {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::Address
  }
}

impl<T: MoveType> MoveType for Vec<T> {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Vector(Box::new(T::move_type(package)))
  }
}
