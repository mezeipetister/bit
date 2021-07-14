use bit::{
  ledger::{self, Ledger},
  project::Project,
};
use chrono::{Datelike, NaiveDate, Utc};
use std::{
  collections::HashMap,
  env,
  error::Error,
  fs,
  path::{Path, PathBuf},
  time::SystemTime,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "BIT",
  about = "(Balance it) double entry book keeping software.\nCopyright (C) 2021 Peter Mezei."
)]
enum Command {
  #[structopt(about = "Create new BIT project")]
  New,
  #[structopt(about = "Generate report about project")]
  Report,
}

fn main() -> Result<(), Box<dyn Error>> {
  let opt: Command = Command::from_args();
  let project = Project::new()?;

  match opt {
    Command::New => todo!(),
    Command::Report => {
      let ledger = project.inspect()?;
      println!("{:?}", ledger);
    }
  }

  Ok(())
}
