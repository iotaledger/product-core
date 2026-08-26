// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use iota_sdk::{
    move_types::iota_framework::object::UID,
    types::{ObjectId, TypeTag},
};
use serde::{Deserialize, Deserializer};

use crate::{network::Network, product_client::ProductClient};

pub trait MoveType {
    fn move_type(client: &impl ProductClient) -> TypeTag;
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("the exact move type of '{type_}' is unknown for network '{network}'")]
#[non_exhaustive]
pub struct UnknownTypeForNetwork {
    pub type_: Cow<'static, str>,
    pub network: Network,
}

impl UnknownTypeForNetwork {
    pub fn new(type_: impl Into<Cow<'static, str>>, network: Network) -> Self {
        Self {
            type_: type_.into(),
            network,
        }
    }
}

pub fn deserialize_object_id_from_uid<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: Deserializer<'de>,
{
    UID::deserialize(deserializer).map(|uid| *uid.object_id())
}
