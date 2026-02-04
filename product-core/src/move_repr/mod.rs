// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uid {
    pub id: ObjectId,
}

impl From<Uid> for ObjectId {
    fn from(uid: Uid) -> Self {
        uid.id
    }
}

pub fn deserialize_object_id_from_uid<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let uid = Uid::deserialize(deserializer)?;
    Ok(uid.into())
}
