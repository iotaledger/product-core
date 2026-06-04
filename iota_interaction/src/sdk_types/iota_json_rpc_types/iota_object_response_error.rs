// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::{
    base_types::{ObjectID, SequenceNumber},
    digests::ObjectDigest,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, IntoStaticStr};
use thiserror::Error;

#[derive(
    Eq,
    PartialEq,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Hash,
    AsRefStr,
    IntoStaticStr,
    Error,
)]
#[serde(tag = "code", rename = "ObjectResponseError", rename_all = "camelCase")]
pub enum IotaObjectResponseError {
    #[error("Object {object_id} does not exist")]
    NotExists {
        object_id: ObjectID,
    },
    #[error("Cannot find dynamic field for parent object {parent_object_id}")]
    DynamicFieldNotFound {
        parent_object_id: ObjectID,
    },
    #[error(
        "Object has been deleted object_id: {object_id} at version: {version} in digest {digest}"
    )]
    Deleted {
        object_id: ObjectID,
        /// Object version.
        version: SequenceNumber,
        /// Base64 string representing the object digest
        digest: ObjectDigest,
    },
    #[error("Unknown Error")]
    Unknown,
    #[error("Display Error: {error}")]
    Display { error: String },
}
