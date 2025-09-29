// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use iota_interaction::types::base_types::ObjectID;

use super::package_registry::{Env, PackageRegistry};

/// Helper function to extract an ID field from a TOML table with proper error handling.
///
/// # Arguments
/// * `table` - The TOML table to extract the value from.
/// * `key` - The key to extract from the table.
/// * `alias` - The environment alias for error context.
///
/// # Returns
/// The extracted string value or an error with context.
fn get_id_from_table(table: &toml::Table, key: &str, alias: &str) -> Result<String> {
  Ok(
    table
      .get(key)
      .with_context(|| format!("invalid Move.lock file: missing `{key}` for env {alias}"))?
      .as_str()
      .with_context(|| format!("invalid Move.lock file: `{key}` for env {alias} is not a string"))?
      .to_string(),
  )
}

impl PackageRegistry {
  /// Creates a [`PackageRegistry`] from the content of a `Move.lock` file.
  ///
  /// # Arguments
  /// * `move_lock` - A string containing the content of the `Move.lock` file.
  /// * `aliases_to_watch` - A vector of environment aliases to include in the registry. Only environments with aliases
  ///   in this list will be processed and added to the registry. Other environments in the `Move.lock` file will be
  ///   ignored.
  ///
  /// # Returns
  /// A `PackageRegistry` instance populated with data from the `Move.lock` file.
  ///
  /// # Errors
  /// Returns an error if the `Move.lock` file content is invalid or cannot be parsed.
  pub fn from_move_lock_content(move_lock: &str, aliases_to_watch: &[String]) -> anyhow::Result<Self> {
    let mut move_lock: toml::Table = move_lock.parse()?;

    let mut move_lock_iter = move_lock
      .remove("env")
      .context("invalid Move.lock file: missing `env` table")?
      .as_table_mut()
      .map(std::mem::take)
      .context("invalid Move.lock file: `env` is not a table")?
      .into_iter();

    move_lock_iter.try_fold(Self::default(), |mut registry, (alias, table)| {
      if !aliases_to_watch.contains(&alias) {
        return Ok(registry);
      }
      let toml::Value::Table(mut table) = table else {
        anyhow::bail!("invalid Move.lock file: invalid `env` table");
      };
      let chain_id: String = table
        .remove("chain-id")
        .with_context(|| format!("invalid Move.lock file: missing `chain-id` for env {alias}"))?
        .try_into()
        .context("invalid Move.lock file: invalid `chain-id`")?;

      let original_published_id: String = get_id_from_table(&table, "original-published-id", &alias)?;
      let latest_published_id: String = get_id_from_table(&table, "latest-published-id", &alias)?;

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

/// Manages the content of `Move.history.json` files, including initialization and updates.
/// Provides the main functionality needed to implement a `build.rs` script in IOTA product repositories.
///
/// Libraries in IOTA product repositories, depending on Move packages **provided in the same repository**,
/// should have a `build.rs` script (contained in the libraries root folder), that manages the content of
/// the `Move.history.json` file.
///
/// ## `Move.history.json` file
/// The `Move.history.json` file is used to store the history of package versions for Move packages
/// that the library depends on. See the `PackageRegistry` documentation for more details.
///
/// The `Move.history.json` file:
/// * should be located in the same directory as the `Move.lock` file of the Move package that the library depends on
/// * contains all data that is provided by the `PackageRegistry`, which is used by the library to interact with the
///   Move package
/// * will be integrated into build rust binaries at build time (using include_str!()) when the library is built
/// * should not contain the package versions of the `localnet` environment, as this would probably blow up the size of
///   the file and is not needed for production use cases
/// * should be added to the git repository also containing the library and the Move package
/// * should be updated by a `build.rs` script in the library package, whenever the `Move.lock` file of the Move package
///   changes - see below for a `build.rs`  example using the `MoveHistoryManager`
///
/// ## `build.rs` scripts
/// The `MoveHistoryManager` is designed to be used in a `build.rs` script of a library that depends on a Move package
/// provided in the same repository. The `build.rs` script described in the example below will be build and
/// executed every time when the library, containing the `build.rs` script, is built and the `Move.lock`
/// file of the Move package has changed.
///
/// When the library is built and the timestamp of the `Move.lock` file has changed, the `MoveHistoryManager`
/// will check if the `Move.lock` file exists and the `Move.history.json` file will be:
/// * created, if a `Move.lock` file exists, but the `Move.history.json` file does not exist yet
/// * updated, if both the `Move.lock` file and the `Move.history.json` file exist
///
/// If the `Move.lock` file doesn't exist the whole processing is skipped.
///
/// ## Example `build.rs` script
/// This example shows how to use the `MoveHistoryManager` in a `build.rs` script.
///
/// * Please replace `<Move-Package-Name>` with the actual name of the Move package
/// * In this example, the Move package is expected to be located in the parent directory of the library package. If
///   this is not the case, please edit the file paths accordingly
///
/// ``` ignore
/// use std::path::PathBuf;
///
/// use product_common::move_history_manager::MoveHistoryManager;
///
/// fn main() {
///   let move_lock_path = "../<Move-Package-Name>/Move.lock";
///   println!("[build.rs] move_lock_path: {move_lock_path}");
///   let move_history_path = "../<Move-Package-Name>/Move.history.json";
///   println!("[build.rs] move_history_path: {move_history_path}");
///
///   MoveHistoryManager::new(
///     &PathBuf::from(move_lock_path),
///     &PathBuf::from(move_history_path),
///     // We will watch the default watch list (`get_default_aliases_to_watch()`) in this build script
///     // so we leave the `additional_aliases_to_watch` argument vec empty.
///     // Use for example `vec!["localnet".to_string()]` instead, if you don't want to ignore `localnet`.
///     vec![],
///   )
///   .manage_history_file(|message| {
///     println!("[build.rs] {}", message);
///   })
///   .expect("Successfully managed Move history file");
///
///   // Tell Cargo to rerun this build script if the Move.lock file changes.
///   println!("cargo::rerun-if-changed={move_lock_path}");
/// }
/// ```
///
/// To use the `MoveHistoryManager` in a `build.rs` script in an IOTA product library package, you need
/// to add the following build dependency in the `cargo.toml` of the crate containing the `build.rs`
///  file:
///
/// ``` toml
/// [build-dependencies]
/// product_common = { workspace = true, default-features = false, features = ["move-history-manager"] }
/// ```
#[derive(Debug)]
pub struct MoveHistoryManager {
  move_lock_path: PathBuf,
  history_file_path: PathBuf,
  aliases_to_watch: Vec<String>,
}

impl MoveHistoryManager {
  /// Creates a new `MoveHistoryManager` instance.
  ///
  /// # Arguments
  /// * `move_lock_path` - Path to the `Move.lock` file.
  /// * `history_file_path` - Path to the `M̀ove.history.toml` file.
  /// * `additional_aliases_to_watch` - List of environment aliases to be watched additionally to those environments,
  ///   being watched per default (see function `get_default_aliases_to_watch()` for more details). Examples:
  ///   * Watch only defaults environments: `new(move_lock_path, history_file_path, vec![])`
  ///   * Additionally watch the `localnet` environment: `new(move_lock_path, history_file_path, vec!["localnet"])`
  ///
  /// # Returns
  /// A new `MoveHistoryManager` instance.
  ///
  /// Doesn't check if any of the provided paths are invalid or if the `Move.lock` file cannot be parsed.
  /// Functions `manage_history_file`, `init` and `update` will handle those checks.
  pub fn new(move_lock_path: &Path, history_file_path: &Path, mut additional_aliases_to_watch: Vec<String>) -> Self {
    let mut aliases_to_watch = Self::get_default_aliases_to_watch();
    aliases_to_watch.append(&mut additional_aliases_to_watch);

    Self {
      move_lock_path: move_lock_path.to_owned(),
      history_file_path: history_file_path.to_owned(),
      aliases_to_watch,
    }
  }

  /// Returns the default list of environment aliases being watched if no additional
  /// `additional_aliases_to_watch` is provided in the `new()` function.
  /// Returns a vector containing `mainnet`, `testnet` and `devnet`.
  ///
  /// Use the `additional_aliases_to_watch` argument of the `new()` function to specify aliases
  /// of additional environments to be watched, e.g. `localnet`.
  ///
  /// Use method `aliases_to_watch()` to evaluate the complete list of environment aliases being watched
  /// by a `MoveHistoryManager` instance.
  pub fn get_default_aliases_to_watch() -> Vec<String> {
    vec!["mainnet".to_string(), "testnet".to_string(), "devnet".to_string()]
  }

  /// Returns the list of environment aliases being watched by this `MoveHistoryManager` instance.
  pub fn aliases_to_watch(&self) -> &Vec<String> {
    &self.aliases_to_watch
  }

  /// Checks if the Move.history.json file exists.
  pub fn history_file_exists(&self) -> bool {
    self.history_file_path.exists() && self.history_file_path.is_file()
  }

  /// Checks if the Move.lock file exists.
  pub fn move_lock_file_exists(&self) -> bool {
    self.move_lock_path.exists() && self.move_lock_path.is_file()
  }

  /// Returns the path to the Move.lock file.
  pub fn move_lock_path(&self) -> &Path {
    &self.move_lock_path
  }

  /// Returns the path to the Move.history.json file.
  pub fn history_file_path(&self) -> &Path {
    &self.history_file_path
  }

  /// Manages the Move history file by either initializing a new one or updating an existing one
  /// based on the current Move.lock file.
  ///
  /// This method checks for the existence of both the Move.lock and Move history files,
  /// and performs the appropriate action:
  /// - If Move.lock exists and the history file exists: Updates the history file
  /// - If Move.lock exists but the history file doesn't: Creates a new history file
  /// - If Move.lock doesn't exist: Skips any action
  ///
  /// Progress messages can be printed to the app console during the operation via the callback function
  /// provided using the `console_out` argument.
  ///
  /// # Arguments
  /// * `console_out` - Can be used to output status messages in the app console. It should be a closure that takes a
  ///   `String` as an argument. Example: `|message| { println!("{}", message); }`
  /// # Returns
  /// A `Result` that indicates success or contains an error if something went wrong during the process.
  ///
  /// # Errors
  /// This method may return errors from the underlying `init()` or `update()` functions
  /// if there are issues reading or writing files.
  pub fn manage_history_file(&self, console_out: impl Fn(String)) -> anyhow::Result<()> {
    let move_history_path = self.history_file_path.to_string_lossy();
    let move_lock_path = self.move_lock_path.to_string_lossy();

    if !self.move_lock_file_exists() {
      console_out(format!("File `{move_lock_path}` does not exist, skipping..."));
      return Ok(())
    }

    // The move_lock_file exists
    if self.history_file_exists() {
      // If the output file already exists, update it.
      console_out(format!("File `{move_history_path}` already exists, updating..."));
      self.update()?;
      console_out(format!(
        "Successfully updated`{move_history_path}` with content of `{move_lock_path}`"
      ));
    } else {
      // If the output file does not exist, create it.
      console_out(format!("File `{move_history_path}` does not exist, creating..."));
      self.init()?;
      console_out(format!(
        "Successfully created file `{move_history_path}` with content of `{move_lock_path}` content"
      ));
    }
    Ok(())
  }

  /// Creates an initial Move.history.json file from a Move.lock file
  /// Will only take those environment aliases into account, listed in `aliases_to_watch()`.
  pub fn init(&self) -> anyhow::Result<()> {
    let move_lock_content = fs::read_to_string(&self.move_lock_path)
      .with_context(|| format!("Failed to read Move.lock file: {}", &self.move_lock_path.display()))?;

    let registry = PackageRegistry::from_move_lock_content(&move_lock_content, &self.aliases_to_watch)
      .context("Failed to parse Move.lock file")?;

    let json_content = serde_json::to_string_pretty(&registry)?;

    fs::write(&self.history_file_path, json_content)
      .with_context(|| format!("Failed to write to output file: {}", self.history_file_path.display()))?;

    Ok(())
  }

  /// Updates an existing Move.history.json file with new package versions from a Move.lock file
  pub fn update(&self) -> anyhow::Result<()> {
    // Read and deserialize existing package history
    let history_content = fs::read_to_string(&self.history_file_path).with_context(|| {
      format!(
        "Failed to read Move.history.json file: {}",
        self.history_file_path.display()
      )
    })?;

    let mut registry = PackageRegistry::from_package_history_json_str(&history_content)
      .context("Failed to parse existing Move.history.json file")?;

    // Read and parse Move.lock file
    let move_lock_content = fs::read_to_string(&self.move_lock_path)
      .with_context(|| format!("Failed to read Move.lock file: {}", self.move_lock_path.display()))?;

    let new_registry = PackageRegistry::from_move_lock_content(&move_lock_content, &self.aliases_to_watch)
      .context("Failed to parse Move.lock file")?;

    // Add new package versions from Move.lock to existing registry
    for (chain_id, versions) in new_registry.envs().iter() {
      if let Some(latest_version) = versions.last() {
        registry.insert_new_package_version(chain_id, *latest_version);
      }
    }

    // Serialize and write updated registry
    let updated_json_content = serde_json::to_string_pretty(&registry)?;

    fs::write(&self.history_file_path, updated_json_content).with_context(|| {
      format!(
        "Failed to write updated content to: {}",
        self.history_file_path.display()
      )
    })?;

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

  enum InitialTestFile {
    None,
    MoveLock,
    HistoryFile,
  }

  fn setup_missing_history_file_test(
    history_path: &str,
    move_lock_path: &str,
    initial_file: InitialTestFile,
  ) -> (TempDir, PathBuf, PathBuf, MoveHistoryManager) {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join(history_path);
    let move_lock_path = temp_dir.path().join(move_lock_path);

    match initial_file {
      InitialTestFile::None => {
        // Do not create any initial files
      }
      InitialTestFile::MoveLock => {
        fs::write(&move_lock_path, create_test_move_lock()).unwrap();
      }
      InitialTestFile::HistoryFile => {
        fs::write(&history_path, create_test_package_history()).unwrap();
      }
    }

    let history_manager = MoveHistoryManager::new(&move_lock_path, &history_path, vec![]);
    (temp_dir, history_path, move_lock_path, history_manager)
  }

  #[test]
  fn manage_history_file_creates_new_file_when_move_lock_exists_and_history_file_does_not() {
    let (_temp_dir, history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::MoveLock);

    history_manager
      .manage_history_file(|message| {
        println!("{}", message);
      })
      .unwrap();

    assert!(history_path.exists());
    let content = fs::read_to_string(&history_path).unwrap();
    assert!(content.contains("\"aliases\": {"));
    assert!(content.contains("\"mainnet\": \"6364aad5\""));
    assert!(content.contains("\"testnet\": \"2304aa97\""));
    // localnet should not be included per default
    assert!(!content.contains("\"localnet\": \"ecc0606a\""));
  }

  #[test]
  fn manage_history_file_updates_existing_file_when_both_files_exist() {
    let (_temp_dir, history_path, move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::HistoryFile);

    let updated_move_lock = r#"
[env.mainnet]
chain-id = "6364aad5"
latest-published-id = "0x94cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de09"
original-published-id = "0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"
"#;
    fs::write(&move_lock_path, updated_move_lock).unwrap();

    history_manager
      .manage_history_file(|message| {
        println!("{}", message);
      })
      .unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    assert_eq!(registry.history("6364aad5").unwrap().len(), 2);
  }

  #[test]
  fn manage_history_file_skips_action_when_move_lock_does_not_exist() {
    let (_temp_dir, history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "nonexistent.lock", InitialTestFile::None);

    history_manager
      .manage_history_file(|message| {
        println!("{}", message);
      })
      .unwrap();

    assert!(!history_path.exists());
  }

  #[test]
  fn init_creates_package_history_from_move_lock() {
    let (_temp_dir, output_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::MoveLock);

    history_manager.init().unwrap();

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
    let (_temp_dir, _history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("output.json", "nonexistent.lock", InitialTestFile::None);

    let result = history_manager.init();
    assert!(result.is_err());
  }

  #[test]
  fn update_adds_new_package_versions() {
    let (_temp_dir, history_path, move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::HistoryFile);

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

    history_manager.update().unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    assert_eq!(registry.history("6364aad5").unwrap().len(), 2);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 2);
    assert_eq!(registry.history("ecc0606a"), None);
  }

  #[test]
  fn update_fails_with_nonexistent_history_file() {
    let (_temp_dir, _history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("nonexistent.json", "Move.lock", InitialTestFile::MoveLock);

    let result = history_manager.update();
    assert!(result.is_err());
  }

  #[test]
  fn update_does_not_duplicate_same_package_version() {
    let (_temp_dir, history_path, move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::HistoryFile);

    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    history_manager.update().unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    // Should still have only 1 version each since we're adding the same versions
    assert_eq!(registry.history("6364aad5").unwrap().len(), 1);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 1);
  }

  #[test]
  fn history_file_exists_returns_true_when_file_exists() {
    let (_temp_dir, _history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("Move.history.json", "Move.lock", InitialTestFile::HistoryFile);

    assert!(history_manager.history_file_exists());
  }

  #[test]
  fn history_file_exists_returns_false_when_file_does_not_exist() {
    let (_temp_dir, _history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("nonexistent.json", "Move.lock", InitialTestFile::None);

    assert!(!history_manager.history_file_exists());
  }

  #[test]
  fn history_file_exists_returns_false_when_path_is_directory() {
    let (_temp_dir, history_path, _move_lock_path, history_manager) =
      setup_missing_history_file_test("directory", "Move.lock", InitialTestFile::None);

    // Create a directory instead of a file
    fs::create_dir(&history_path).unwrap();
    assert!(!history_manager.history_file_exists());
  }

  #[test]
  fn new_includes_additional_aliases_to_watch() {
    let move_lock_path = PathBuf::from("Move.lock");
    let history_file_path = PathBuf::from("Move.history.json");
    let additional = vec!["localnet".to_string(), "customnet".to_string()];
    let manager = MoveHistoryManager::new(&move_lock_path, &history_file_path, additional.clone());

    let mut expected = MoveHistoryManager::get_default_aliases_to_watch();
    expected.extend(additional);

    assert_eq!(manager.aliases_to_watch(), &expected);
  }

  #[test]
  fn aliases_to_watch_returns_only_defaults_when_no_additional_provided() {
    let move_lock_path = PathBuf::from("Move.lock");
    let history_file_path = PathBuf::from("Move.history.json");
    let manager = MoveHistoryManager::new(&move_lock_path, &history_file_path, vec![]);

    let expected = MoveHistoryManager::get_default_aliases_to_watch();
    assert_eq!(manager.aliases_to_watch(), &expected);
  }
}
