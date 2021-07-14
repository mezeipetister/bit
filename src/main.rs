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
  io::{self, BufRead, Write},
  path::{Path, PathBuf},
  time::SystemTime,
  usize,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "BIT",
  about = "(Balance it) double entry book keeping software.\nCopyright (C) 2021 Peter Mezei."
)]
enum Command {
  #[structopt(about = "Create new BIT project")]
  New(NewOpt),
  #[structopt(about = "Generate report about project")]
  Report,
  Ledger(LedgerOpt),
}

#[derive(Debug, StructOpt)]
pub struct NewOpt {
  pub project_folder_name: String,
}

#[derive(Debug, StructOpt)]
pub struct LedgerOpt {
  pub date: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
  let opt: Command = Command::from_args();

  match opt {
    Command::New(nopt) => {
      let mut line = String::new();
      let stdin = io::stdin();

      // Read project name
      print!("Project name: ");
      io::stdout().flush().unwrap();
      stdin.lock().read_line(&mut line).unwrap();
      let name = line.trim_end().to_owned();
      line.clear();

      // Read project desc
      print!("Project desc: ");
      io::stdout().flush().unwrap();
      stdin.lock().read_line(&mut line).unwrap();
      let desc = line.trim_end().to_owned();
      line.clear();

      // Read project currency
      print!("Project currency: ");
      io::stdout().flush().unwrap();
      stdin.lock().read_line(&mut line).unwrap();
      let currency = line.trim_end().to_owned();
      line.clear();

      Project::new_project(&nopt.project_folder_name, name, desc, currency)?;
      println!("Project created");
    }
    Command::Report => {
      // Init project
      let project = Project::new()?;
      let ledger = project.inspect()?;
      println!("{:?}", ledger);
    }
    Command::Ledger(lopt) => {
      // Init project
      let project = Project::new()?;
      // Define day
      let day = match lopt.date {
        Some(d) => d
          .parse::<NaiveDate>()
          .map_err(|_| "Could not parse ledger date".to_string())?,
        None => Utc::today().naive_local(),
      };
      // Define day index
      let day_index = day.ordinal0() as usize;
      // Try inspect and get the given day
      let ledger = project.inspect()?.get_ledger_by_date(day_index)?;
      println!("\nLedger for date: {}\n", day);
      println!(
        "{0: <10} {1: <10} | {2: <10} | {3: <10} | {4: <10}",
        "Accounts", "T. Debit", "T. Credit", "B. Debit", "B. Credit"
      );
      println!(
        "{}",
        "-----------------------------------------------------------"
      );
      // Print result
      ledger
        .iter()
        .for_each(|day| println!("{0: <10} {1: <10}", day.0, day.1.print_full()));
    }
  }

  Ok(())
}
