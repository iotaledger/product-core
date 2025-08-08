// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

use iota_interaction::types::base_types::ObjectID;

use super::package_registry::{Env, PackageRegistry};

fn remove_first_and_last_char_from_string(value: String) -> String {
  let mut chars = value.chars();
  chars.next();
  chars.next_back();
  chars.as_str().to_string()
}

fn to_prettified_string(registry: &PackageRegistry) -> anyhow::Result<String> {
  let json_value = serde_json::to_value(registry).context("Failed to serialize PackageRegistry to JSON value")?;
  Ok(format!("{json_value:#}"))
}

impl PackageRegistry {
  /// Creates a [`PackageRegistry`] from the content of a `Move.lock` file.
  ///
  /// # Arguments
  /// * `move_lock` - A string containing the content of the `Move.lock` file.
  ///
  /// # Returns
  /// A `PackageRegistry` instance populated with data from the `Move.lock` file.
  ///
  /// # Errors
  /// Returns an error if the `Move.lock` file content is invalid or cannot be parsed.
  pub fn from_move_lock_content(move_lock: &str) -> anyhow::Result<Self> {
    let mut move_lock: toml::Table = move_lock.parse()?;

    let mut move_lock_iter = move_lock
      .remove("env")
      .context("invalid Move.lock file: missing `env` table")?
      .as_table_mut()
      .map(std::mem::take)
      .context("invalid Move.lock file: `env` is not a table")?
      .into_iter();

    move_lock_iter.try_fold(Self::default(), |mut registry, (alias, table)| {
      let toml::Value::Table(mut table) = table else {
        anyhow::bail!("invalid Move.lock file: invalid `env` table");
      };
      let chain_id: String = table
        .remove("chain-id")
        .context(format!("invalid Move.lock file: missing `chain-id` for env {alias}"))?
        .try_into()
        .context("invalid Move.lock file: invalid `chain-id`")?;

      let original_published_id: String = remove_first_and_last_char_from_string(
        table
          .get("original-published-id")
          .context(format!(
            "invalid Move.lock file: missing `original-published-id` for env {alias}"
          ))?
          .to_string(),
      );
      let latest_published_id: String = remove_first_and_last_char_from_string(
        table
          .get("latest-published-id")
          .context(format!(
            "invalid Move.lock file: missing `latest-published-id` for env {alias}"
          ))?
          .to_string(),
      );

      let mut metadata = vec![ObjectID::from_hex_literal(original_published_id.as_str())?];
      if original_published_id != latest_published_id {
        metadata.push(ObjectID::from_hex_literal(latest_published_id.as_str())?);
      }

      let env = Env::new_with_alias(chain_id, alias.clone());
      registry.insert_env(env, metadata);

      Ok(registry)
    })
  }
}

/// Manages the history of Move packages, including initialization and updates.
pub struct MoveHistoryManager {
  move_lock_path: PathBuf,
  history_file_path: PathBuf,
  aliases_to_ignore: Vec<String>,
}

impl MoveHistoryManager {
  /// Creates a new `MoveHistoryManager` instance.
  ///
  /// # Arguments
  /// * `move_lock_path` - Path to the `Move.lock` file.
  /// * `history_file_path` - Path to the `M̀ove.history.toml` file.
  /// * `aliases_to_ignore` - Optional list of environment aliases to ignore.
  ///    If `aliases_to_ignore` is not provided, it defaults to `["localnet"]`.
  ///
  /// # Returns
  /// A new `MoveHistoryManager` instance.
  ///
  /// Doesn't check if any of the provided paths are invalid or if the `Move.lock` file cannot be parsed.
  /// Functions `init` and `update` will handle those checks.
  pub fn new(move_lock_path: &Path, history_file_path: &Path, aliases_to_ignore: Option<Vec<String>>) -> Self {
    let aliases_to_ignore = aliases_to_ignore
        .unwrap_or(vec!["localnet".to_string()]);
    Self {
      move_lock_path: move_lock_path.to_owned(),
      history_file_path: history_file_path.to_owned(),
      aliases_to_ignore,
    }
  }

  /// Checks if the Move.history.json file exists.
  pub fn history_file_exists(&self) -> bool {
    self.history_file_path.exists() && self.history_file_path.is_file()
  }

  /// Returns the list of environment aliases to ignore.
  pub fn aliases_to_ignore(&self) -> &[String] {
    &self.aliases_to_ignore
  }

  /// Returns the path to the Move.lock file.
  pub fn move_lock_path(&self) -> &Path {
    &self.move_lock_path
  }

  /// Returns the path to the Move.history.json file.
  pub fn history_file_path(&self) -> &Path {
    &self.history_file_path
  }

  /// Creates an initial Move.history.json file from a Move.lock file
  /// Will ignore any environment aliases specified in `aliases_to_ignore`.
  pub fn init(&self) -> anyhow::Result<()> {
    let move_lock_content = fs::read_to_string(&self.move_lock_path)
      .with_context(|| format!("Failed to read Move.lock file: {}", &self.move_lock_path.display()))?;

    let mut registry =
      PackageRegistry::from_move_lock_content(&move_lock_content).context("Failed to parse Move.lock file")?;

    for alias in self.aliases_to_ignore.iter() {
      let _ = registry.remove_env_by_alias(alias);
    }

    let json_content = to_prettified_string(&registry)?;

    fs::write(&self.history_file_path, json_content)
      .with_context(|| format!("Failed to write to output file: {}", self.history_file_path.display()))?;

    Ok(())
  }

  /// Updates an existing Move.history.json file with new package versions from a Move.lock file
  pub fn update(&self) -> anyhow::Result<()> {
    // Read and deserialize existing package history
    let history_content = fs::read_to_string(&self.history_file_path)
      .with_context(|| format!("Failed to read Move.history.json file: {}", self.history_file_path.display()))?;

    let mut registry = PackageRegistry::from_package_history_json_str(&history_content)
      .context("Failed to parse existing Move.history.json file")?;

    // Read and parse Move.lock file
    let move_lock_content = fs::read_to_string(&self.move_lock_path)
      .with_context(|| format!("Failed to read Move.lock file: {}", self.move_lock_path.display()))?;

    let mut new_registry =
      PackageRegistry::from_move_lock_content(&move_lock_content).context("Failed to parse Move.lock file")?;

    for alias in self.aliases_to_ignore.iter() {
      let _ = new_registry.remove_env_by_alias(alias);
    }

    // Add new package versions from Move.lock to existing registry
    for (chain_id, versions) in new_registry.envs().iter() {
      if let Some(latest_version) = versions.last() {
        registry.insert_new_package_version(chain_id, *latest_version);
      }
    }

    // Serialize and write updated registry
    let updated_json_content = to_prettified_string(&registry)?;

    fs::write(&self.history_file_path, updated_json_content)
      .with_context(|| format!("Failed to write updated content to: {}", self.history_file_path.display()))?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::fs;

  use tempfile::TempDir;

  use super::*;

  fn create_test_move_lock() -> String {
    r#"
[env.mainnet]
chain-id = "6364aad5"
original-published-id = "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
latest-published-id = "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"

[env.testnet]
chain-id = "2304aa97"
original-published-id = "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"
latest-published-id = "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"

[env.localnet]
chain-id = "ecc0606a"
original-published-id = "0xfbddb4631d027b2c4f0b4b90c020713d258ed32bdb342b5397f4da71edb7478b"
latest-published-id = "0xfbddb4631d027b2c4f0b4b90c020713d258ed32bdb342b5397f4da71edb7478b"
"#
    .to_string()
  }

  fn create_test_package_history() -> String {
    r#"
{
  "aliases": {
    "testnet": "2304aa97",
    "mainnet": "6364aad5"
  },
  "envs": {
    "6364aad5": ["0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"],
    "2304aa97": ["0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"]
  }
}
"#
    .to_string()
  }

  #[test]
  fn init_creates_package_history_from_move_lock() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("Move.lock");
    let output_path = temp_dir.path().join("Move.history.json");

    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    MoveHistoryManager::new(&move_lock_path, &output_path, None).init().unwrap();

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("\"aliases\": {"));
    assert!(content.contains("\"mainnet\": \"6364aad5\""));
    assert!(content.contains("\"testnet\": \"2304aa97\""));
    assert!(!content.contains("\"localnet\": \"ecc0606a\""));

    assert!(content.contains("\"envs\": {"));
    assert!(content.contains("\"2304aa97\": ["));
    assert!(content.contains("\"0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555\""));
    assert!(content.contains("\"6364aad5\": ["));
    assert!(content.contains("\"0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08\""));
    assert!(!content.contains("\"ecc0606a\": ["));
    assert!(!content.contains("\"0xfbddb4631d027b2c4f0b4b90c020713d258ed32bdb342b5397f4da71edb7478b\""));
  }

  #[test]
  fn init_fails_with_nonexistent_move_lock() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("nonexistent.lock");
    let output_path = temp_dir.path().join("output.json");

    let result = MoveHistoryManager::new(&move_lock_path, &output_path, None).init();
    assert!(result.is_err());
  }

  #[test]
  fn update_adds_new_package_versions() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("Move.history.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&history_path, create_test_package_history()).unwrap();

    let updated_move_lock = r#"
[env.mainnet]
chain-id = "6364aad5"
latest-published-id = "0x94cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de09"
original-published-id = "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"

[env.testnet]
chain-id = "2304aa97"
latest-published-id = "0x332741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc666"
original-published-id = "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"

[env.localnet]
chain-id = "ecc0606a"
original-published-id = "0xfbddb4631d027b2c4f0b4b90c020713d258ed32bdb342b5397f4da71edb7478b"
latest-published-id = "0x0d88bcecde97585d50207a029a85d7ea0bacf73ab741cbaa975a6e279251033a"
"#;
    fs::write(&move_lock_path, updated_move_lock).unwrap();

    MoveHistoryManager::new(&move_lock_path, &history_path, None).update().unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    assert_eq!(registry.history("6364aad5").unwrap().len(), 2);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 2);
    assert_eq!(registry.history("ecc0606a"), None);
  }

  #[test]
  fn update_fails_with_nonexistent_history_file() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("nonexistent.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    let result = MoveHistoryManager::new(&move_lock_path, &history_path, None).update();
    assert!(result.is_err());
  }

  #[test]
  fn update_does_not_duplicate_same_package_version() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("Move.history.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&history_path, create_test_package_history()).unwrap();
    fs::write(&move_lock_path, create_test_move_lock()).unwrap();
    MoveHistoryManager::new(&move_lock_path, &history_path, None).update().unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    // Should still have only 1 version each since we're adding the same versions
    assert_eq!(registry.history("6364aad5").unwrap().len(), 1);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 1);
  }

  #[test]
  fn history_file_exists_returns_true_when_file_exists() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("Move.lock");
    let history_path = temp_dir.path().join("Move.history.json");

    // Create the history file
    fs::write(&history_path, "{}").unwrap();

    let manager = MoveHistoryManager::new(&move_lock_path, &history_path, None);
    assert!(manager.history_file_exists());
  }

  #[test]
  fn history_file_exists_returns_false_when_file_does_not_exist() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("Move.lock");
    let history_path = temp_dir.path().join("nonexistent.json");

    let manager = MoveHistoryManager::new(&move_lock_path, &history_path, None);
    assert!(!manager.history_file_exists());
  }

  #[test]
  fn history_file_exists_returns_false_when_path_is_directory() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("Move.lock");
    let history_path = temp_dir.path().join("directory");

    // Create a directory instead of a file
    fs::create_dir(&history_path).unwrap();

    let manager = MoveHistoryManager::new(&move_lock_path, &history_path, None);
    assert!(!manager.history_file_exists());
  }
}