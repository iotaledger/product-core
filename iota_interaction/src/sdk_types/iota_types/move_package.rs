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

pub use iota_sdk_types::move_package::{MovePackage, TypeOrigin, UpgradeInfo};
use iota_sdk_types::Identifier;

pub const PACKAGE_METADATA_MODULE_NAME: Identifier = Identifier::from_static("package_metadata");
pub const PACKAGE_METADATA_V1_STRUCT_NAME: Identifier =
    Identifier::from_static("PackageMetadataV1");
pub const PACKAGE_METADATA_KEY_STRUCT_NAME: Identifier =
    Identifier::from_static("PackageMetadataKey");