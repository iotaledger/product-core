// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

pub use iota_sdk_types::{MoveStruct as MoveObject, ObjectData as Data, Owner};
use serde::{Deserialize, Serialize};

use super::base_types::{ObjectDigest, ObjectID, SequenceNumber, TransactionDigest};
use super::crypto::default_hash;
pub const OBJECT_START_VERSION: SequenceNumber = SequenceNumber::from_u64(1);

/// Index marking the end of the object's ID + the beginning of its version
pub const ID_END_INDEX: usize = ObjectID::LENGTH;

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename = "Object")]
pub struct ObjectInner {
    /// The meat of the object
    pub data: Data,
    /// The owner that unlocks this object
    pub owner: Owner,
    /// The digest of the transaction that created or last mutated this object
    pub previous_transaction: TransactionDigest,
    /// The amount of IOTA we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    pub storage_rebate: u64,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(from = "ObjectInner")]
pub struct Object(Arc<ObjectInner>);

impl From<ObjectInner> for Object {
    fn from(inner: ObjectInner) -> Self {
        Self(Arc::new(inner))
    }
}

impl Object {
    pub fn into_inner(self) -> ObjectInner {
        match Arc::try_unwrap(self.0) {
            Ok(inner) => inner,
            Err(inner_arc) => (*inner_arc).clone(),
        }
    }

    pub fn as_inner(&self) -> &ObjectInner {
        &self.0
    }

    pub fn owner(&self) -> &Owner {
        &self.0.owner
    }
}

impl std::ops::Deref for Object {
    type Target = ObjectInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl ObjectInner {
    pub fn digest(&self) -> ObjectDigest {
        ObjectDigest::new(default_hash(self))
    }

    pub fn id(&self) -> ObjectID {
        use Data::*;

        match &self.data {
            Struct(v) => v.id(),
            Package(m) => m.id(),
        }
    }

    pub fn version(&self) -> SequenceNumber {
        use Data::*;

        match &self.data {
            Struct(o) => o.version(),
            Package(p) => p.version(),
        }
    }
}