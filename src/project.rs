use crate::{file, file_index::FileIndex, inspector::Inspector, ledger::Ledger};
use serde::Deserialize;
use std::{
  env,
  path::{Path, PathBuf},
};

/// BIT Config
#[derive(Deserialize, Debug)]
pub struct Config {
  pub name: String,
  pub description: Option<String>,
  pub year: String,
  pub currency: String,
  pub dependencies: Dependencies,
}

#[derive(Deserialize, Debug)]
pub struct Dependencies {
  pub accounts: String,
  pub logs: String,
}

pub struct Project {
  project_root_path: PathBuf,
  config: Config,
  accounts_path: PathBuf,
  // balance_path: PathBuf,
  // profit_path: PathBuf,
  logs_path: PathBuf,
  file_index: FileIndex,
}

impl Project {
  pub fn new() -> Result<Self, String> {
    // Try to get current working dir
    let current_dir =
      env::current_dir().map_err(|_| "Current working dir does not exist".to_string())?;

    // Try to get project dir
    let project_root_path = get_project_dir(&current_dir)?;

    let index_path = project_root_path.join(".bit");

    // Try load config file
    let cfg_file = project_root_path.join("config.toml");
    // Check if it exist
    match cfg_file.exists() || cfg_file.is_file() {
      true => (),
      false => return Err("No config file found!".to_string()),
    }
    // Try to read its content
    let cfg_content = std::fs::read_to_string(&cfg_file)
      .map_err(|_| "Cannot read config.toml content".to_string())?;
    // Try to deserialize its contetn
    let config: Config = toml::from_str(&cfg_content)
      .map_err(|_| "Error while deserialize config.toml".to_string())?;

    let accounts_path = project_root_path.join(&config.dependencies.accounts);
    // Check depdendencies
    if !accounts_path.exists() && !accounts_path.is_file() {
      return Err("Accounts def file does not exist!".to_string());
    }

    let logs_path = project_root_path.join(&config.dependencies.logs);
    if !logs_path.exists() && !logs_path.is_dir() {
      return Err("Logs path not exist or not a folder!".to_string());
    }

    // Load files index db
    let file_index = FileIndex::load(&index_path);

    Ok(Self {
      project_root_path,
      config,
      accounts_path,
      logs_path,
      file_index,
    })
  }
  fn index_path(&self) -> PathBuf {
    self.project_root_path.join(".bit")
  }
  // Get project files
  pub fn read_files_recurs(&self) -> Vec<crate::file::File> {
    crate::file::read_files_recurs(&self.project_root_path)
  }

  // Inspect files
  pub fn inspect(&self) -> Result<Ledger, String> {
    let mut ledger = Ledger::new();

    // Inspect accounts
    Inspector::init_from_file(&self.accounts_path)?.process_to_ledger(&mut ledger)?;

    // Inspect logs
    for f in file::read_files_recurs(&self.logs_path) {
      Inspector::init_from_file(&f.path)?.process_to_ledger(&mut ledger)?;
    }

    Ok(ledger)
  }
}

// Try to get BIT project root path
fn get_project_dir(dir: &Path) -> Result<PathBuf, String> {
  let p = dir.join(".bit");
  match p.exists() && p.is_dir() {
    true => Ok(dir.to_path_buf()),
    false => get_project_dir(
      dir
        .parent()
        .ok_or("Given directory is not a BIT working directory".to_string())?,
    ),
  }
}
