use std::{
  collections::HashMap,
  env,
  error::Error,
  fs,
  path::{Path, PathBuf},
  time::SystemTime,
};

use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "BIT",
  about = "(Balance it) double entry book keeping software.\nCopyright (C) 2021 Peter Mezei."
)]
enum Command {
  #[structopt(about = "Create new BIT project")]
  New,
  #[structopt(about = "Add file(s) to project")]
  Add,
  #[structopt(about = "Remove file(s) from project")]
  Remove,
  #[structopt(about = "Revert file(s)")]
  Revert,
  #[structopt(about = "Commit changes to project")]
  Commit,
  #[structopt(about = "Generate report about project")]
  Report,
}

#[derive(Debug, StructOpt)]
enum Transaction {
  Add(TransactionAdd),
  Find,
}

#[derive(Debug, StructOpt)]
struct TransactionAdd {
  #[structopt(short, long)]
  event: String,
  #[structopt(short, long)]
  debit: String,
  #[structopt(short, long)]
  credit: String,
  #[structopt(short, long)]
  amount: i32,
  #[structopt(short, long)]
  name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
  pub name: String,
  pub description: Option<String>,
  pub year: String,
  pub currency: String,
  pub dependencies: Dependencies,
}

#[derive(Deserialize, Debug)]
struct Dependencies {
  accounts: String,
  balance_sheet: String,
  profit_loss: String,
  logs: String,
}

#[derive(Debug)]
struct Project {
  current_dir: PathBuf,
  project_dir: PathBuf,
  index_dir: PathBuf,
  config: Config,
}

impl Project {
  fn new() -> Result<Self, String> {
    // Try to get current working dir
    let current_dir =
      env::current_dir().map_err(|_| "Current working dir does not exist".to_string())?;

    // Try to get project dir
    let project_dir = get_project_dir(&current_dir)?;

    let index_dir = project_dir.join(".bit");

    // Try load config file
    let cfg_file = project_dir.join("config.toml");
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

    // Check depdendencies
    if !project_dir.join(&config.dependencies.accounts).exists() {
      return Err("Accounts def file does not exist!".to_string());
    }

    if !project_dir
      .join(&config.dependencies.balance_sheet)
      .exists()
    {
      return Err("Balance def file does not exist!".to_string());
    }

    if !project_dir.join(&config.dependencies.profit_loss).exists() {
      return Err("Profit and loss file does not exist!".to_string());
    }

    Ok(Self {
      current_dir,
      project_dir,
      index_dir,
      config,
    })
  }
  // Get project files
  pub fn read_files_recurs(&self) -> Vec<bit::file::File> {
    bit::file::read_files_recurs(&self.project_dir)
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

fn main() -> Result<(), Box<dyn Error>> {
  let opt = Command::from_args();
  let project = Project::new().unwrap();
  println!("Project file is: {:?}", &project);
  println!("Project files: {:?}", project.read_files_recurs());
  Ok(())
}
