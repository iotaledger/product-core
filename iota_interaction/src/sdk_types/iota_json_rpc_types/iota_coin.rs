// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde_with::DisplayFromStr;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::super::iota_types::base_types::{ObjectID, ObjectRef, SequenceNumber, TransactionDigest};
use super::super::iota_types::digests::ObjectDigest;
use super::{
    Page,
    iota_primitives::SequenceNumberString as SequenceNumberStringSchema,
};

pub type CoinPage = Page<Coin, ObjectID>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub coin_type: String,
    pub coin_object_id: ObjectID,
    #[serde_as(as = "SequenceNumberStringSchema")]
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u64,
    pub previous_transaction: TransactionDigest,
}

impl Coin {
    pub fn object_ref(&self) -> ObjectRef {
        ObjectRef::new(self.coin_object_id, self.version, self.digest)
    }
}
