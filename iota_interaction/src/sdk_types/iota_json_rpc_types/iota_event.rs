// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use serde_json::Value;

use fastcrypto::encoding::{Base58, Base64};

use super::super::iota_types::{
    base_types::{ObjectID, IotaAddress, TransactionDigest},
    event::EventID,
    iota_serde::{BigInt, IotaStructTag}
};
use super::super::move_core_types::{
    identifier::Identifier,
    language_storage::{StructTag},
};

use super::{Page};

pub type EventPage = Page<IotaEvent, EventID>;

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "Event", rename_all = "camelCase")]
pub struct IotaEvent {
    /// Sequential event ID, ie (transaction seq number, event seq number).
    /// 1) Serves as a unique event ID for each fullnode
    /// 2) Also serves to sequence events for the purposes of pagination and
    ///    querying. A higher id is an event seen later by that fullnode.
    /// This ID is the "cursor" for event querying.
    pub id: EventID,
    /// Move package where this event was emitted.
    pub package_id: ObjectID,
    #[serde_as(as = "DisplayFromStr")]
    /// Move module where this event was emitted.
    pub transaction_module: Identifier,
    /// Sender's IOTA address.
    pub sender: IotaAddress,
    #[serde_as(as = "IotaStructTag")]
    /// Move event type.
    pub type_: StructTag,
    /// Parsed json value of the event
    pub parsed_json: Value,
    /// Base64 encoded bcs bytes of the move event
    #[serde(flatten)]
    pub bcs: BcsEvent,
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub timestamp_ms: Option<u64>,
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "bcsEncoding")]
#[serde(from = "MaybeTaggedBcsEvent")]
pub enum BcsEvent {
    Base64 {
        #[serde_as(as = "Base64")]
        bcs: Vec<u8>,
    },
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

impl BcsEvent {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self::Base64 { bcs: bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            BcsEvent::Base64 { bcs } => bcs.as_ref(),
            BcsEvent::Base58 { bcs } => bcs.as_ref(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            BcsEvent::Base64 { bcs } => bcs,
            BcsEvent::Base58 { bcs } => bcs,
        }
    }
}

#[allow(unused)]
#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum MaybeTaggedBcsEvent {
    Tagged(TaggedBcsEvent),
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "bcsEncoding")]
enum TaggedBcsEvent {
    Base64 {
        #[serde_as(as = "Base64")]
        bcs: Vec<u8>,
    },
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

impl From<MaybeTaggedBcsEvent> for BcsEvent {
    fn from(event: MaybeTaggedBcsEvent) -> BcsEvent {
        let bcs = match event {
            MaybeTaggedBcsEvent::Tagged(TaggedBcsEvent::Base58 { bcs })
            | MaybeTaggedBcsEvent::Base58 { bcs } => bcs,
            MaybeTaggedBcsEvent::Tagged(TaggedBcsEvent::Base64 { bcs }) => bcs,
        };

        // Bytes are already decoded, force into Base64 variant to avoid serializing to
        // base58
        Self::Base64 { bcs }
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventFilter {
    /// Query by sender address.
    Sender(IotaAddress),
    /// Return events emitted by the given transaction.
    Transaction(
        /// digest of the transaction, as base-64 encoded string
        TransactionDigest,
    ),
    /// Return events emitted in a specified Package.
    Package(ObjectID),
    /// Return events emitted in a specified Move module.
    /// If the event is defined in Module A but emitted in a tx with Module B,
    /// query `MoveModule` by module B returns the event.
    /// Query `MoveEventModule` by module A returns the event too.
    MoveModule {
        /// the Move package ID
        package: ObjectID,
        /// the module name
        #[serde_as(as = "DisplayFromStr")]
        module: Identifier,
    },
    /// Return events with the given Move event struct name (struct tag).
    /// For example, if the event is defined in `0xabcd::MyModule`, and named
    /// `Foo`, then the struct tag is `0xabcd::MyModule::Foo`.
    MoveEventType(
        #[serde_as(as = "IotaStructTag")]
        StructTag,
    ),
    /// Return events with the given Move module name where the event struct is
    /// defined. If the event is defined in Module A but emitted in a tx
    /// with Module B, query `MoveEventModule` by module A returns the
    /// event. Query `MoveModule` by module B returns the event too.
    MoveEventModule {
        /// the Move package ID
        package: ObjectID,
        /// the module name
        #[serde_as(as = "DisplayFromStr")]
        module: Identifier,
    },
    MoveEventField {
        path: String,
        value: Value,
    },
    /// Return events emitted in [start_time, end_time] interval
    #[serde(rename_all = "camelCase")]
    TimeRange {
        /// left endpoint of time interval, milliseconds since epoch, inclusive
        #[serde_as(as = "BigInt<u64>")]
        start_time: u64,
        /// right endpoint of time interval, milliseconds since epoch, exclusive
        #[serde_as(as = "BigInt<u64>")]
        end_time: u64,
    },

    All(Vec<EventFilter>),
    Any(Vec<EventFilter>),
    And(Box<EventFilter>, Box<EventFilter>),
    Or(Box<EventFilter>, Box<EventFilter>),
}

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}