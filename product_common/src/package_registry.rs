// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use iota_interaction::types::base_types::ObjectID;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const MAINNET_CHAIN_ID: &str = "6364aad5";

/// Network / Chain information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Env {
  pub chain_id: String,
  pub alias: Option<String>,
}

impl Env {
  /// Creates a new package's environment.
  pub fn new(chain_id: impl Into<String>) -> Self {
    Self {
      chain_id: chain_id.into(),
      alias: None,
    }
  }

  /// Creates a new package's environment with the given alias.
  pub fn new_with_alias(chain_id: impl Into<String>, alias: impl Into<String>) -> Self {
    Self {
      chain_id: chain_id.into(),
      alias: Some(alias.into()),
    }
  }
}

/// A registry that tracks package versions across different blockchain environments.
///
/// The `PackageRegistry` stores:
/// - Aliases that map human-readable network names (like "mainnet", "testnet") to chain IDs.
/// - Environment mappings that associate chain IDs with the history of package versions. The history of package
///   versions is ordered chronologically, with the latest version at the end of the array.
///
/// # Initialization using `Move.history.json` files
///
/// The registry can be initialized from a `Move.history.json` file using the function
/// `from_package_history_json_str()`. A `Move.history.json` file has the following structure:
/// ```json
/// {
///   "aliases": {
///     "networkName": "chainId",
///     // e.g., "mainnet": "6364aad5"
///   },
///   "envs": {
///     "chainId": ["0xpackageId1", "0xpackageId2"],
///     // Where the last ID is the most recent version
///   }
/// }
/// ```
/// `Move.history.json` files can automatically be generated and updated using `build.rs`
/// scripts in your Rust projects. The `product_common` crate provides a `MoveHistoryManager`
/// that can be used to manage the `Move.history.json` file. See there for more details.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageRegistry {
  aliases: HashMap<String, String>,
  envs: HashMap<String, Vec<ObjectID>>,
}

impl PackageRegistry {
  /// Returns the historical list of this package's versions for a given `chain`.
  /// `chain` can either be a chain identifier or its alias.
  ///
  /// ID at position `0` is the first ever published version of the package, `1` is
  /// the second, and so forth until the last, which is the currently active version.
  pub fn history(&self, chain: &str) -> Option<&[ObjectID]> {
    let from_alias = || self.aliases.get(chain).and_then(|chain_id| self.envs.get(chain_id));
    self.envs.get(chain).or_else(from_alias).map(|v| v.as_slice())
  }

  /// Returns this package's latest version ID for a given chain.
  pub fn package_id(&self, chain: &str) -> Option<ObjectID> {
    self.history(chain).and_then(|versions| versions.last()).copied()
  }

  /// Returns the alias of a given chain-id.
  pub fn chain_alias(&self, chain_id: &str) -> Option<&str> {
    self
      .aliases
      .iter()
      .find_map(|(alias, chain)| (chain == chain_id).then_some(alias.as_str()))
  }

  /// Removes the environment specified by the alias from the registry.
  /// Returns the removed environment's versions if it existed, or `None` if the alias was not found.
  pub fn remove_env_by_alias(&mut self, alias: &str) -> Option<Vec<ObjectID>> {
    if let Some(chain_id) = self.aliases.remove(alias) {
      self.envs.remove(&chain_id)
    } else {
      None
    }
  }

  /// Returns the envs of this package registry.
  pub fn envs(&self) -> &HashMap<String, Vec<ObjectID>> {
    &self.envs
  }

  /// Adds or replaces this package's metadata for a given environment.
  pub fn insert_env(&mut self, env: Env, metadata: Vec<ObjectID>) {
    let Env { chain_id, alias } = env;

    if let Some(alias) = alias {
      self.aliases.insert(alias, chain_id.clone());
    }
    self.envs.insert(chain_id, metadata);
  }

  /// Inserts a new package version for a given chain.
  pub fn insert_new_package_version(&mut self, chain_id: &str, package: ObjectID) {
    let history = self.envs.entry(chain_id.to_string()).or_default();
    if history.last() != Some(&package) {
      history.push(package)
    }
  }

  /// Merges another [PackageRegistry] into this one.
  pub fn join(&mut self, other: PackageRegistry) {
    self.aliases.extend(other.aliases);
    self.envs.extend(other.envs);
  }

  /// Creates a [PackageRegistry] from a Move.history.json file.
  pub fn from_package_history_json_str(package_history: &str) -> anyhow::Result<Self> {
    let package_history: Value = serde_json::from_str(package_history)?;

    let ret_val = package_history
      .get("aliases")
      .context("invalid Move.history.json file: missing `aliases` object")?
      .as_object()
      .context("invalid Move.history.json file: `aliases` is not a JSON object literal")?
      .into_iter()
      .try_fold(Self::default(), |mut registry, (alias, chain_id)| {
        let chain_id: String = chain_id
          .as_str()
          .context(format!(
            "invalid Move.history.json file: invalid `chain-id` '{chain_id}' for alias {alias}"
          ))?
          .to_string();
        registry.aliases.insert(alias.clone(), chain_id);
        Ok::<PackageRegistry, anyhow::Error>(registry)
      })?;

    package_history
      .get("envs")
      .context("invalid Move.history.json file: missing `envs` object")?
      .as_object()
      .context("invalid Move.history.json file: `envs` is not a JSON object literal")?
      .into_iter()
      .try_fold(ret_val, |mut registry, (chain_id, versions)| {
        let versions: Vec<ObjectID> = versions
          .as_array()
          .context(format!("invalid Move.history.json file: invalid versions for {chain_id}. versions is not an array"))?
          .iter()
          .try_fold(Vec::<ObjectID>::new(), |mut arr, v| {
            let obj_id = ObjectID::from_hex_literal(
              v.as_str()
                  .context(format!("invalid Move.history.json file: invalid versions array element for {chain_id}. Elements need to be strings"))?
            )?;
            arr.push(obj_id);
            Ok::<Vec<ObjectID>, anyhow::Error>(arr)
          })?;
        registry.envs.insert(chain_id.clone(), versions);
        Ok(registry)
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! object_id {
    ($id:literal) => {
      ObjectID::from_hex_literal($id).unwrap()
    };
  }

  const PACKAGE_HISTORY_JSON: &str = r#"
{
  "aliases": {
    "localnet": "594fb3ed",
    "devnet": "e678123a",
    "testnet": "2304aa97",
    "mainnet": "6364aad5"
  },
  "envs": {
    "6364aad5": ["0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"],
    "e678123a": [
      "0xe6fa03d273131066036f1d2d4c3d919b9abbca93910769f26a924c7a01811103",
      "0x6a976d3da90db5d27f8a0c13b3268a37e582b455cfc7bf72d6461f6e8f668823"
    ],
    "594fb3ed": ["0xd097794267324a58734ff754919f4a16461e39ed39901b29778b86a1261930ba"],
    "2304aa97": [
      "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555",
      "0x3403da7ec4cd2ff9bdf6f34c0b8df5a2bd62c798089feb0d2ebf1c2e953296dc"
    ]
  }
}
"#;

  #[test]
  fn deserialize_package_registry_from_valid_json() {
    let registry = PackageRegistry::from_package_history_json_str(PACKAGE_HISTORY_JSON).unwrap();
    assert_eq!(registry.aliases.get("mainnet"), Some(&"6364aad5".to_string()));
    assert_eq!(registry.aliases.get("testnet"), Some(&"2304aa97".to_string()));
    assert_eq!(registry.envs.get("6364aad5").unwrap().len(), 1);
    assert_eq!(registry.envs.get("2304aa97").unwrap().len(), 2);
    assert_eq!(registry.history("mainnet").unwrap().len(), 1);
    assert_eq!(registry.history("testnet").unwrap().len(), 2);
    assert_eq!(
      registry.history("testnet").unwrap()[0],
      object_id!("0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555")
    );
    assert_eq!(
      registry.history("testnet").unwrap()[1],
      object_id!("0x3403da7ec4cd2ff9bdf6f34c0b8df5a2bd62c798089feb0d2ebf1c2e953296dc")
    );
    assert_eq!(
      registry.package_id("mainnet"),
      Some(object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      ))
    );
    assert_eq!(
      registry.package_id("testnet"),
      Some(object_id!(
        "0x3403da7ec4cd2ff9bdf6f34c0b8df5a2bd62c798089feb0d2ebf1c2e953296dc"
      ))
    );
    assert_eq!(
      registry.package_id("devnet"),
      Some(object_id!(
        "0x6a976d3da90db5d27f8a0c13b3268a37e582b455cfc7bf72d6461f6e8f668823"
      ))
    );
  }

  #[test]
  fn package_id_returns_correct_id() {
    let registry = PackageRegistry::from_package_history_json_str(PACKAGE_HISTORY_JSON).unwrap();
    let package_id = registry.package_id("mainnet");
    assert_eq!(
      package_id,
      Some(object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      ))
    );
  }

  #[test]
  fn test_serialize_package_registry_to_json() {
    let mut registry = PackageRegistry::default();
    // Add well-known networks.
    registry.insert_env(
      Env::new_with_alias("6364aad5", "mainnet"),
      vec![object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      )],
    );
    registry.insert_env(
      Env::new_with_alias("2304aa97", "testnet"),
      vec![
        object_id!("0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"),
        object_id!("0x3403da7ec4cd2ff9bdf6f34c0b8df5a2bd62c798089feb0d2ebf1c2e953296dc"),
      ],
    );
    registry.insert_env(
      Env::new_with_alias("e678123a", "devnet"),
      vec![
        object_id!("0xe6fa03d273131066036f1d2d4c3d919b9abbca93910769f26a924c7a01811103"),
        object_id!("0x6a976d3da90db5d27f8a0c13b3268a37e582b455cfc7bf72d6461f6e8f668823"),
      ],
    );

    let json_content = serde_json::to_string(&registry).unwrap();
    let _ = PackageRegistry::from_package_history_json_str(json_content.as_str())
      .expect("Serialized json string can be deserialized back to PackageRegistry");
  }

  #[test]
  fn package_id_returns_none_for_unknown_chain() {
    let registry = PackageRegistry::from_package_history_json_str(PACKAGE_HISTORY_JSON).unwrap();
    let package_id = registry.package_id("unknown_chain");
    assert_eq!(package_id, None);
  }

  #[test]
  fn chain_alias_returns_none_for_unknown_chain_id() {
    let registry = PackageRegistry::from_package_history_json_str(PACKAGE_HISTORY_JSON).unwrap();
    let alias = registry.chain_alias("unknown_chain_id");
    assert_eq!(alias, None);
  }

  #[test]
  fn insert_env_overwrites_existing_alias() {
    let mut registry = PackageRegistry::default();
    registry.insert_env(
      Env::new_with_alias("6364aad5", "mainnet"),
      vec![object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      )],
    );
    registry.insert_env(
      Env::new_with_alias("2304aa97", "mainnet"),
      vec![object_id!(
        "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"
      )],
    );
    assert_eq!(registry.aliases.get("mainnet"), Some(&"2304aa97".to_string()));
  }

  #[test]
  fn insert_new_package_version_does_not_duplicate_last_version() {
    let mut registry = PackageRegistry::default();
    registry.insert_new_package_version(
      "6364aad5",
      object_id!("0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"),
    );
    registry.insert_new_package_version(
      "6364aad5",
      object_id!("0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"),
    );
    assert_eq!(
      registry.history("6364aad5").unwrap(),
      &[object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      )]
    );
  }

  #[test]
  fn join_merges_aliases_and_envs() {
    let mut registry1 = PackageRegistry::default();
    registry1.insert_env(
      Env::new_with_alias("6364aad5", "mainnet"),
      vec![object_id!(
        "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
      )],
    );

    let mut registry2 = PackageRegistry::default();
    registry2.insert_env(
      Env::new_with_alias("2304aa97", "testnet"),
      vec![object_id!(
        "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"
      )],
    );

    registry1.join(registry2);

    assert_eq!(registry1.aliases.get("mainnet"), Some(&"6364aad5".to_string()));
    assert_eq!(registry1.aliases.get("testnet"), Some(&"2304aa97".to_string()));
    assert!(registry1.envs.contains_key("6364aad5"));
    assert!(registry1.envs.contains_key("2304aa97"));
  }
}
