// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde_with::DisplayFromStr;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use iota_sdk_types::ObjectId;
use super::super::iota_types::base_types::{ObjectRef, SequenceNumber, TransactionDigest};
use super::super::iota_types::digests::ObjectDigest;
use super::{
    Page,
    iota_primitives::{
        Base58 as Base58Schema, ObjectId as ObjectIdSchema,
        SequenceNumberString as SequenceNumberStringSchema,
    },
};

pub type CoinPage = Page<Coin, ObjectId>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub coin_type: String,
    #[serde_as(as = "ObjectIdSchema")]
    pub coin_object_id: ObjectId,
    #[serde_as(as = "SequenceNumberStringSchema")]
    pub version: SequenceNumber,
    #[serde_as(as = "Base58Schema")]
    pub digest: ObjectDigest,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u64,
    #[serde_as(as = "Base58Schema")]
    pub previous_transaction: TransactionDigest,
}

impl Coin {
    pub fn object_ref(&self) -> ObjectRef {
        ObjectRef::new(self.coin_object_id, self.version, self.digest)
    }
}
