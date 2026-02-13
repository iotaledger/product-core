// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Move package.
//!
//! This module contains the [MovePackage] and types necessary for describing
//! its update behavior and linkage information for module resolution during
//! execution.
//!
//! Upgradeable packages form a version chain. This is simply the conceptual
//! chain of package versions, with their monotonically increasing version
//! numbers. Package { version: 1 } => Package { version: 2 } => ...
//!
//! The code contains terminology that may be confusing for the uninitiated,
//! like `Module ID`, `Package ID`, `Storage ID` and `Runtime ID`. For avoidance
//! of doubt these concepts are defined like so:
//! - `Package ID` is the [ObjectID] representing the address by which the given
//!   package may be found in storage.
//! - `Runtime ID` will always mean the `Package ID`/`Storage ID` of the
//!   initially published package. For a non upgradeable package this will
//!   always be equal to `Storage ID`. For an upgradeable package, it will be
//!   the `Storage ID` of the package's first deployed version.
//! - `Storage ID` is the `Package ID`, and it is mostly used in to highlight
//!   that we are talking about the current `Package ID` and not the `Runtime
//!   ID`
//! - `Module ID` is the the type
//!   [ModuleID](move_core_types::language_storage::ModuleId).
//!
//! Some of these are redundant and have overlapping meaning, so whenever
//! reasonable/necessary the possible naming will be listed. From all of these
//! `Runtime ID` and `Module ID` are the most confusing. `Module ID` may be used
//! with `Runtime ID` and `Storage ID` depending on the context. While `Runtime
//! ID` is mostly used in name resolution during runtime, when a package with
//! its modules has been loaded.
use std::{
    collections::BTreeMap,
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use crate::ident_str;
use super::base_types::{ObjectID, SequenceNumber};
use super::super::move_core_types::identifier::IdentStr;

pub const PACKAGE_MODULE_NAME: &IdentStr = ident_str!("package");
pub const UPGRADECAP_STRUCT_NAME: &IdentStr = ident_str!("UpgradeCap");
pub const UPGRADETICKET_STRUCT_NAME: &IdentStr = ident_str!("UpgradeTicket");
pub const UPGRADERECEIPT_STRUCT_NAME: &IdentStr = ident_str!("UpgradeReceipt");

pub const PACKAGE_METADATA_MODULE_NAME: &IdentStr = ident_str!("package_metadata");
pub const PACKAGE_METADATA_V1_STRUCT_NAME: &IdentStr = ident_str!("PackageMetadataV1");
pub const PACKAGE_METADATA_KEY_STRUCT_NAME: &IdentStr = ident_str!("PackageMetadataKey");

/// Store the origin of a data type where it first appeared in the version
/// chain.
///
/// A data type is identified by the name of the module and the name of the
/// struct/enum in combination.
///
/// # Undefined behavior
///
/// Directly modifying any field is undefined behavior. The fields are only
/// public for read-only access.
#[derive(
Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
pub struct TypeOrigin {
    /// The name of the module the data type resides in.
    pub module_name: String,
    /// The name of the data type.
    ///
    /// Here this either refers to an enum or a struct identifier.
    // `struct_name` alias to support backwards compatibility with the old name
    #[serde(alias = "struct_name")]
    pub datatype_name: String,
    /// `Storage ID` of the package, where the given type first appeared.
    pub package: ObjectID,
}

/// Value for the [MovePackage]'s linkage_table.
///
/// # Undefined behavior
///
/// Directly modifying any field is undefined behavior. The fields are only
/// public for read-only access.
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct UpgradeInfo {
    /// `Storage ID`/`Package ID` of the referred package.
    pub upgraded_id: ObjectID,
    /// The version of the package at `upgraded_id`.
    pub upgraded_version: SequenceNumber,
}

// serde_bytes::ByteBuf is an analog of Vec<u8> with built-in fast
// serialization.
#[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MovePackage {
    /// The `Storage ID` of the package.
    pub(crate) id: ObjectID,
    /// Most move packages are uniquely identified by their ID (i.e. there is
    /// only one version per ID), but the version is still stored because
    /// one package may be an upgrade of another (at a different ID), in
    /// which case its version will be one greater than the version of the
    /// upgraded package.
    ///
    /// Framework packages are an exception to this rule -- all versions of the
    /// framework packages exist at the same ID, at increasing versions.
    ///
    /// In all cases, packages are referred to by move calls using just their
    /// ID, and they are always loaded at their latest version.
    pub(crate) version: SequenceNumber,
    /// Map module identifiers to their serialized [CompiledModule].
    ///
    /// All modules within a package share the `Storage ID` of their containing
    /// package.
    #[serde_as(as = "BTreeMap<_, Bytes>")]
    pub(crate) module_map: BTreeMap<String, Vec<u8>>,

    /// Maps structs and enums in a given module to a package version where they
    /// were first defined.
    ///
    /// Stored as a vector for simple serialization and
    /// deserialization.
    pub(crate) type_origin_table: Vec<TypeOrigin>,

    /// For each dependency, it maps the `Runtime ID` (the first package's
    /// `Storage ID` in a version chain) of the containing package to the
    /// `UpgradeInfo` containing the actually used version.
    pub(crate) linkage_table: BTreeMap<ObjectID, UpgradeInfo>,
}

impl MovePackage {
    /// `Package ID`/`Storage ID` of this package.
    pub fn id(&self) -> ObjectID {
        self.id
    }

    pub fn version(&self) -> SequenceNumber {
        self.version
    }

    pub fn serialized_module_map(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.module_map
    }

    pub fn type_origin_table(&self) -> &Vec<TypeOrigin> {
        &self.type_origin_table
    }

    pub fn linkage_table(&self) -> &BTreeMap<ObjectID, UpgradeInfo> {
        &self.linkage_table
    }
}
