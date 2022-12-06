use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::cmd::Cmd;

mod cmd;
// mod cmd_parser;
mod commands;
mod rpc;
mod context;
mod core;
mod db;
mod fs;
mod index;
mod message;
mod prelude;

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
