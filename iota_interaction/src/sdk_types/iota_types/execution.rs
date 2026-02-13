// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use super::{
    base_types::SequenceNumber,
    digests::{ObjectDigest, TransactionDigest},
    object::Owner,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DynamicallyLoadedObjectMetadata {
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub owner: Owner,
    pub storage_rebate: u64,
    pub previous_transaction: TransactionDigest,
}