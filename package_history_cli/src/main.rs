// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use product_common::package_registry::PackageRegistry;

#[derive(Parser)]
#[command(name = "package-history")]
#[command(about = "A CLI tool for managing Move package history files")]
#[command(version = "1.0")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Create an initial Move.package-history.json file from a Move.lock file
  Init {
    /// Path to the Move.lock file
    #[arg(short, long, default_value = "Move.lock")]
    move_lock: PathBuf,
    /// Output path for the Move.package-history.json file
    #[arg(short, long, default_value = "Move.package-history.json")]
    output: PathBuf,
  },
  /// Add a new package version to an existing Move.package-history.json file
  Update {
    /// Path to the existing Move.package-history.json file
    #[arg(short('f'), long, default_value = "Move.package-history.json")]
    history_file: PathBuf,
    /// Path to the Move.lock file containing the new version
    #[arg(short, long)]
    move_lock: Option<PathBuf>,
  },
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::Init { move_lock, output } => {
      init_package_history(&move_lock, &output)?;
    }
    Commands::Update {
      history_file,
      move_lock,
    } => {
      let move_lock_path =
        move_lock.unwrap_or_else(|| history_file.parent().unwrap_or(Path::new(".")).join("Move.lock"));
      update_package_history(&history_file, &move_lock_path)?;
    }
  }

  Ok(())
}

/// Creates an initial Move.package-history.json file from a Move.lock file
fn init_package_history(move_lock_path: &Path, output_path: &Path) -> Result<()> {
  let move_lock_content = fs::read_to_string(move_lock_path)
    .with_context(|| format!("Failed to read Move.lock file: {}", move_lock_path.display()))?;

  let registry =
    PackageRegistry::from_move_lock_content(&move_lock_content).context("Failed to parse Move.lock file")?;

  let json_content = to_prettified_string(&registry)?;

  fs::write(output_path, json_content)
    .with_context(|| format!("Failed to write to output file: {}", output_path.display()))?;

  println!(
    "Successfully created Move.package-history.json from {}",
    move_lock_path.display()
  );

  Ok(())
}

fn to_prettified_string(registry: &PackageRegistry) -> Result<String> {
  let json_value = serde_json::to_value(registry).context("Failed to serialize PackageRegistry to JSON value")?;
  Ok(format!("{json_value:#}"))
}

/// Updates an existing Move.package-history.json file with new package versions from a Move.lock file
fn update_package_history(history_file_path: &Path, move_lock_path: &Path) -> Result<()> {
  // Read and deserialize existing package history
  let history_content = fs::read_to_string(history_file_path).with_context(|| {
    format!(
      "Failed to read Move.package-history.json file: {}",
      history_file_path.display()
    )
  })?;

  let mut registry = PackageRegistry::from_package_history_json_str(&history_content)
    .context("Failed to parse existing Move.package-history.json file")?;

  // Create backup file
  create_backup_file(history_file_path)?;

  // Read and parse Move.lock file
  let move_lock_content = fs::read_to_string(move_lock_path)
    .with_context(|| format!("Failed to read Move.lock file: {}", move_lock_path.display()))?;

  let new_registry =
    PackageRegistry::from_move_lock_content(&move_lock_content).context("Failed to parse Move.lock file")?;

  // Add new package versions from Move.lock to existing registry
  for (chain_id, versions) in new_registry.envs().iter() {
    if let Some(latest_version) = versions.last() {
      registry.insert_new_package_version(chain_id, *latest_version);
    }
  }

  // Serialize and write updated registry
  let updated_json_content = to_prettified_string(&registry)?;

  fs::write(history_file_path, updated_json_content)
    .with_context(|| format!("Failed to write updated content to: {}", history_file_path.display()))?;

  println!(
    "Successfully updated {} with new versions from {}",
    history_file_path.display(),
    move_lock_path.display()
  );

  Ok(())
}

/// Creates a backup file with timestamp
fn create_backup_file(original_path: &Path) -> Result<()> {
  let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
  let backup_path = original_path.with_extension(format!("json.bak-{timestamp}"));

  fs::copy(original_path, &backup_path)
    .with_context(|| format!("Failed to create backup file: {}", backup_path.display()))?;

  println!("Created backup file: {}", backup_path.display());
  Ok(())
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

[env.testnet]
chain-id = "2304aa97"
original-published-id = "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"
"#
    .to_string()
  }

  fn create_test_package_history() -> String {
    r#"
[aliases]
mainnet = "6364aad5"
testnet = "2304aa97"

[envs]
6364aad5 = ["0x84cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de08"]
2304aa97 = ["0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"]
"#
    .to_string()
  }

  #[test]
  fn init_creates_package_history_from_move_lock() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("Move.lock");
    let output_path = temp_dir.path().join("Move.package-history.json");

    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    init_package_history(&move_lock_path, &output_path).unwrap();

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("[aliases]"));
    assert!(content.contains("[envs]"));
  }

  #[test]
  fn init_fails_with_nonexistent_move_lock() {
    let temp_dir = TempDir::new().unwrap();
    let move_lock_path = temp_dir.path().join("nonexistent.lock");
    let output_path = temp_dir.path().join("output.json");

    let result = init_package_history(&move_lock_path, &output_path);
    assert!(result.is_err());
  }

  #[test]
  fn update_adds_new_package_versions() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("Move.package-history.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&history_path, create_test_package_history()).unwrap();

    let updated_move_lock = r#"
[env.mainnet]
chain-id = "6364aad5"
original-published-id = "0x94cf5d12de2f9731a89bb519bc0c982a941b319a33abefdd5ed2054ad931de09"

[env.testnet]
chain-id = "2304aa97"
original-published-id = "0x332741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc666"
"#;
    fs::write(&move_lock_path, updated_move_lock).unwrap();

    update_package_history(&history_path, &move_lock_path).unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    assert_eq!(registry.history("6364aad5").unwrap().len(), 2);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 2);
  }

  #[test]
  fn update_creates_backup_file() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("Move.package-history.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&history_path, create_test_package_history()).unwrap();
    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    update_package_history(&history_path, &move_lock_path).unwrap();

    let backup_files: Vec<_> = fs::read_dir(temp_dir.path())
      .unwrap()
      .filter_map(|entry| entry.ok())
      .filter(|entry| {
        entry
          .file_name()
          .to_string_lossy()
          .starts_with("Move.package-history.json.bak-")
      })
      .collect();

    assert_eq!(backup_files.len(), 1);
  }

  #[test]
  fn update_fails_with_nonexistent_history_file() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("nonexistent.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    let result = update_package_history(&history_path, &move_lock_path);
    assert!(result.is_err());
  }

  #[test]
  fn update_does_not_duplicate_same_package_version() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("Move.package-history.json");
    let move_lock_path = temp_dir.path().join("Move.lock");

    fs::write(&history_path, create_test_package_history()).unwrap();
    fs::write(&move_lock_path, create_test_move_lock()).unwrap();

    update_package_history(&history_path, &move_lock_path).unwrap();

    let updated_content = fs::read_to_string(&history_path).unwrap();
    let registry = PackageRegistry::from_package_history_json_str(&updated_content).unwrap();

    // Should still have only 1 version each since we're adding the same versions
    assert_eq!(registry.history("6364aad5").unwrap().len(), 1);
    assert_eq!(registry.history("2304aa97").unwrap().len(), 1);
  }
}
