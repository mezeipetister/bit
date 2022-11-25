use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::cmd::Cmd;

mod cmd;
mod cmd_parser;
mod commands;
mod context;
mod index;

#[derive(Serialize, Deserialize, Debug)]
struct NoteParams {
    id: Option<String>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    idate: Option<NaiveDate>,
    credit: Option<String>,
    debit: Option<String>,
    amount: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = r#"ID r2022-01-0000/asd CREDIT 1 DEBIT 2 AMOUNT 100 CDATE 2022-01-01"#;
    let r: NoteParams = cmd_parser::try_parse(cmd)?;
    println!("{:?}", r);

    // let d: cmd::Demo = cmd::Demo;
    // println!("{}", d.get_keyword());

    Ok(())
}
