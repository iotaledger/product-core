// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::LazyLock;

use iota_interaction::types::base_types::ObjectID;

use crate::package_registry::PackageRegistry;

static TF_COMPONENTS_PACKAGE_REGISTRY: LazyLock<PackageRegistry> = LazyLock::new(|| {
  let package_history_json = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../components_move/Move.history.json"
  ));

  PackageRegistry::from_package_history_json_str(package_history_json)
    .expect("TfComponents Move.history.json exists and is valid")
});

pub fn tf_components_package_id(network: &str) -> Option<ObjectID> {
  TF_COMPONENTS_PACKAGE_REGISTRY.package_id(network)
}
