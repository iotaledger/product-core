// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use iota_sdk::types::base_types::ObjectID;
use serde::{Deserialize, Deserializer, Serialize};

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

/// A published package's metadata for a certain environment.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
  pub original_published_id: ObjectID,
  pub latest_published_id: ObjectID,
  #[serde(deserialize_with = "deserialize_u64_from_str")]
  pub published_version: u64,
}

impl Metadata {
  /// Create a new [Metadata] assuming a newly published package.
  pub fn from_package_id(package: ObjectID) -> Self {
    Self {
      original_published_id: package,
      latest_published_id: package,
      published_version: 1,
    }
  }
}

fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  use serde::de::Error;

  String::deserialize(deserializer)?.parse().map_err(D::Error::custom)
}

#[derive(Debug, Clone, Default)]
pub struct PackageRegistry {
  aliases: HashMap<String, String>,
  envs: HashMap<String, Metadata>,
}

impl PackageRegistry {
  /// Returns the package [Metadata] for a given `chain`.
  /// `chain` can either be a chain identifier or its alias.
  pub fn metadata(&self, chain: &str) -> Option<&Metadata> {
    let from_alias = || self.aliases.get(chain).and_then(|chain_id| self.envs.get(chain_id));
    self.envs.get(chain).or_else(from_alias)
  }

  /// Returns this package's latest version ID for a given chain.
  pub fn package_id(&self, chain: &str) -> Option<ObjectID> {
    self.metadata(chain).map(|meta| meta.latest_published_id)
  }

  /// Returns the alias of a given chain-id.
  pub fn chain_alias(&self, chain_id: &str) -> Option<&str> {
    self
      .aliases
      .iter()
      .find_map(|(alias, chain)| (chain == chain_id).then_some(alias.as_str()))
  }

  /// Adds or replaces this package's metadata for a given environment.
  pub fn insert_env(&mut self, env: Env, metadata: Metadata) {
    let Env { chain_id, alias } = env;

    if let Some(alias) = alias {
      self.aliases.insert(alias, chain_id.clone());
    }
    self.envs.insert(chain_id, metadata);
  }

  /// Merges another [PackageRegistry] into this one.
  pub fn join(&mut self, other: PackageRegistry) {
    self.aliases.extend(other.aliases);
    self.envs.extend(other.envs);
  }

  /// Creates a [PackageRegistry] from a Move.lock file.
  pub fn from_move_lock_content(move_lock: &str) -> anyhow::Result<Self> {
    let mut move_lock: toml::Table = move_lock.parse()?;

    move_lock
      .remove("env")
      .context("invalid Move.lock file: missing `env` table")?
      .as_table_mut()
      .map(std::mem::take)
      .context("invalid Move.lock file: `env` is not a table")?
      .into_iter()
      .try_fold(Self::default(), |mut registry, (alias, table)| {
        let toml::Value::Table(mut table) = table else {
          anyhow::bail!("invalid Move.lock file: invalid `env` table");
        };
        let chain_id: String = table
          .remove("chain-id")
          .context(format!("invalid Move.lock file: missing `chain-id` for env {alias}"))?
          .try_into()
          .context("invalid Move.lock file: invalid `chain-id`")?;

        let env = Env::new_with_alias(chain_id, alias.clone());
        let metadata = table
          .try_into()
          .context(format!("invalid Move.lock file: invalid env metadata for {alias}"))?;
        registry.insert_env(env, metadata);

        Ok(registry)
      })
  }
}
