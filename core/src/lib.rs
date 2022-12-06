use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub mod cmd;
// mod cmd_parser;
pub mod commands;
pub mod context;
pub mod core;
pub mod db;
pub mod fs;
pub mod index;
pub mod message;
pub mod prelude;
pub mod rpc;

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
