// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

use super::base_types::{IotaAddress, MoveObjectType, ObjectID, SequenceNumber, TransactionDigest};
use super::move_package::MovePackage;
use super::error::{IotaError, IotaResult};

pub const OBJECT_START_VERSION: SequenceNumber = SequenceNumber::from_u64(1);

#[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MoveObject {
    /// The type of this object. Immutable
    pub(crate) type_: MoveObjectType,
    /// Number that increases each time a tx takes this object as a mutable
    /// input This is a lamport timestamp, not a sequentially increasing
    /// version
    pub(crate) version: SequenceNumber,
    /// BCS bytes of a Move struct value
    #[serde_as(as = "Bytes")]
    pub(crate) contents: Vec<u8>,
}

/// Index marking the end of the object's ID + the beginning of its version
pub const ID_END_INDEX: usize = ObjectID::LENGTH;

impl MoveObject {
    pub fn type_(&self) -> &MoveObjectType {
        &self.type_
    }


    pub fn contents(&self) -> &[u8] {
        &self.contents
    }

    pub fn into_contents(self) -> Vec<u8> {
        self.contents
    }

    pub fn into_type(self) -> MoveObjectType {
        self.type_
    }

    pub fn into_inner(self) -> (MoveObjectType, Vec<u8>) {
        (self.type_, self.contents)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub enum Data {
    /// An object whose governing logic lives in a published Move module
    Move(MoveObject),
    /// Map from each module name to raw serialized Move module bytes
    Package(MovePackage),
    // ... IOTA "native" types go here
}

#[derive(
Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize, Hash, Ord, PartialOrd, JsonSchema)]
pub enum Owner {
    /// Object is exclusively owned by a single address, and is mutable.
    AddressOwner(IotaAddress),
    /// Object is exclusively owned by a single object, and is mutable.
    /// The object ID is converted to IotaAddress as IotaAddress is universal.
    ObjectOwner(IotaAddress),
    /// Object is shared, can be used by any address, and is mutable.
    Shared {
        /// The version at which the object became shared
        initial_shared_version: SequenceNumber,
    },
    /// Object is immutable, and hence ownership doesn't matter.
    Immutable,
}

impl Owner {
    // NOTE: only return address of AddressOwner, otherwise return error,
    // ObjectOwner's address is converted from object id, thus we will skip it.
    pub fn get_address_owner_address(&self) -> IotaResult<IotaAddress> {
        match self {
            Self::AddressOwner(address) => Ok(*address),
            Self::Shared { .. } | Self::Immutable | Self::ObjectOwner(_) => {
                Err(IotaError::UnexpectedOwnerType)
            }
        }
    }

    // NOTE: this function will return address of both AddressOwner and ObjectOwner,
    // address of ObjectOwner is converted from object id, even though the type is
    // IotaAddress.
    pub fn get_owner_address(&self) -> IotaResult<IotaAddress> {
        match self {
            Self::AddressOwner(address) | Self::ObjectOwner(address) => Ok(*address),
            Self::Shared { .. } | Self::Immutable => Err(IotaError::UnexpectedOwnerType),
        }
    }

    pub fn is_immutable(&self) -> bool {
        matches!(self, Owner::Immutable)
    }

    pub fn is_address_owned(&self) -> bool {
        matches!(self, Owner::AddressOwner(_))
    }

    pub fn is_child_object(&self) -> bool {
        matches!(self, Owner::ObjectOwner(_))
    }

    pub fn is_shared(&self) -> bool {
        matches!(self, Owner::Shared { .. })
    }
}

impl PartialEq<IotaAddress> for Owner {
    fn eq(&self, other: &IotaAddress) -> bool {
        match self {
            Self::AddressOwner(address) => address == other,
            Self::ObjectOwner(_) | Self::Shared { .. } | Self::Immutable => false,
        }
    }
}

impl PartialEq<ObjectID> for Owner {
    fn eq(&self, other: &ObjectID) -> bool {
        let other_id: IotaAddress = (*other).into();
        match self {
            Self::ObjectOwner(id) => id == &other_id,
            Self::AddressOwner(_) | Self::Shared { .. } | Self::Immutable => false,
        }
    }
}

impl Display for Owner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressOwner(address) => {
                write!(f, "Account Address ( {address} )")
            }
            Self::ObjectOwner(address) => {
                write!(f, "Object ID: ( {address} )")
            }
            Self::Immutable => {
                write!(f, "Immutable")
            }
            Self::Shared {
                initial_shared_version,
            } => {
                write!(f, "Shared( {} )", initial_shared_version.value())
            }
        }
    }
}

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