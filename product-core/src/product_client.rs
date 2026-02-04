// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use iota_sdk::{graphql_client::Client as IotaClient, types::ObjectId};

use crate::network::Network;

pub trait Product {}

pub trait ProductClient<P: Product>: Deref<Target = IotaClient> {
    fn network(&self) -> Network;
    fn package_id(&self) -> ObjectId;
}
