// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use iota_sdk::types::TypeTag;

use crate::network::Network;

pub trait MoveType {
    fn move_type(network: Network) -> Result<TypeTag, UnknownTypeForNetwork>;
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
