use bit::{
  ledger::{unzip_dates, LedgerIndexItem},
  project::Project,
};
use chrono::{Datelike, NaiveDate, Utc};
use std::{
  collections::HashMap,
  error::Error,
  io::{self, BufRead, Write},
  usize,
};
use structopt::clap::Shell;
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
  #[structopt(about = "Check project health")]
  Check,
  #[structopt(about = "Get ledger details by date")]
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
  // generate `bash` completions in "target" directory
  // Command::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

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
    Command::Check => {
      let project = Project::new()?;
      let _ = project.inspect()?;
      println!("Project is healthy");
    }
    Command::Ledger(lopt) => {
      // Init project
      let project = Project::new()?;
      // Define day
      let dates = unzip_dates(lopt.date)?;

      // Define first day index
      let fday_index = dates.0.ordinal0() as usize;

      // Define last day index
      let lday_index = dates.1.ordinal0() as usize;

      // Try inspect and get the given day
      let ledger = project.inspect()?;
      let res = (fday_index..lday_index)
        .map(|day_index| {
          ledger
            .get_ledger_by_date(day_index)
            .expect("Cannot get ledger by day index")
        })
        .collect::<Vec<HashMap<String, LedgerIndexItem>>>();

      println!("\nLedger for date: {} - {}\n", dates.0, dates.1);

      println!(
        "{0: <25}  {1: <13} | {2: <13} | {3: <13} | {4: <13}",
        "Account ID", "T. Debit", "T. Credit", "B. Debit", "B. Credit"
      );
      println!(
        "{}",
        "-------------------------------------------------------------------------------------"
      );
      // Print result
      ledger.accounts.iter().for_each(|account| {
        let mut r = res[0]
          .get(&account.id)
          .expect("Cannot get first item")
          .to_owned();

        for day_index in &res[1..] {
          r = r
            + day_index
              .get(&account.id)
              .expect("No data for account")
              .to_owned();
        }

        println!(
          "{0: <5} {1: <20} {2: <13}",
          &account.id,
          account
            .name
            .chars()
            .into_iter()
            .take(15)
            .collect::<String>(),
          r.print_full()
        );
        println!(
          "{}",
          "------------------------------------------------------------------------------------*"
        );
      });
    }
  }

  Ok(())
}
