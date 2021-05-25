use bit::ledger::Ledger;
use chrono::{Datelike, NaiveDate, Utc};
use std::{
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

fn main() -> Result<(), Box<dyn Error>> {
  let today = Utc::today().naive_local();
  println!("Today year day is {}", today.ordinal());
  let last_day = NaiveDate::from_ymd(Utc::today().year(), 12, 31);
  println!("This year has {} days", last_day.ordinal());
  println!("{:?}", Ledger::new());
  let opt = Command::from_args();
  Ok(())
}
