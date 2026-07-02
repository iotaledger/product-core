// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::encoding::{Base58, Base64};
use iota_sdk_types::{Address, Identifier, ObjectId, StructTag};
use crate::types::{
    base_types::TransactionDigest,
    event::EventID,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};

use super::{
    Page,
    iota_primitives::{
        Base58 as Base58Schema, Identifier as IdentifierSchema,
        Address as AddressSchema, ObjectId as ObjectIdSchema, StructTag as StructTagSchema,
    },
};

pub type EventPage = Page<IotaEvent, EventID>;

/// Unique ID of an IOTA Event, the ID is a combination of transaction digest
/// and event seq number.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IotaEventID {
    #[serde_as(as = "Base58Schema")]
    pub tx_digest: TransactionDigest,
    #[serde_as(as = "DisplayFromStr")]
    pub event_seq: u64,
}

impl From<EventID> for IotaEventID {
    fn from(id: EventID) -> Self {
        Self {
            tx_digest: id.tx_digest,
            event_seq: id.event_seq,
        }
    }
}

impl From<IotaEventID> for EventID {
    fn from(id: IotaEventID) -> Self {
        Self {
            tx_digest: id.tx_digest,
            event_seq: id.event_seq,
        }
    }
}

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
    #[serde_as(as = "ObjectIdSchema")]
    pub package_id: ObjectId,
    #[serde_as(as = "IdentifierSchema")]
    /// Move module where this event was emitted.
    pub transaction_module: Identifier,
    /// Sender's IOTA address.
    #[serde_as(as = "AddressSchema")]
    pub sender: Address,
    /// Move event type.
    #[serde_as(as = "StructTagSchema")]
    pub type_: StructTag,
    /// Parsed json value of the event
    pub parsed_json: Value,
    /// Base64 encoded bcs bytes of the move event
    #[serde(flatten)]
    pub bcs: BcsEvent,
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<DisplayFromStr>")]
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
    Sender(
        #[serde_as(as = "AddressSchema")]
        Address,
    ),
    /// Return events emitted by the given transaction.
    Transaction(
        /// digest of the transaction, as base-64 encoded string
        #[serde_as(as = "Base58Schema")]
        TransactionDigest,
    ),
    /// Return events emitted in a specified Package.
    Package(
        #[serde_as(as = "ObjectIdSchema")]
        ObjectId,
    ),
    /// Return events emitted in a specified Move module.
    /// If the event is defined in Module A but emitted in a tx with Module B,
    /// query `MoveModule` by module B returns the event.
    /// Query `MoveEventModule` by module A returns the event too.
    MoveModule {
        /// the Move package ID
        #[serde_as(as = "ObjectIdSchema")]
        package: ObjectId,
        /// the module name
        #[serde_as(as = "IdentifierSchema")]
        module: Identifier,
    },
    /// Return events with the given Move event struct name (struct tag).
    /// For example, if the event is defined in `0xabcd::MyModule`, and named
    /// `Foo`, then the struct tag is `0xabcd::MyModule::Foo`.
    MoveEventType(
        #[serde_as(as = "StructTagSchema")]
        StructTag,
    ),
    /// Return events with the given Move module name where the event struct is
    /// defined. If the event is defined in Module A but emitted in a tx
    /// with Module B, query `MoveEventModule` by module A returns the
    /// event. Query `MoveModule` by module B returns the event too.
    MoveEventModule {
        /// the Move package ID
        #[serde_as(as = "ObjectIdSchema")]
        package: ObjectId,
        /// the module name
        #[serde_as(as = "IdentifierSchema")]
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
        #[serde_as(as = "DisplayFromStr")]
        start_time: u64,
        /// right endpoint of time interval, milliseconds since epoch, exclusive
        #[serde_as(as = "DisplayFromStr")]
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
