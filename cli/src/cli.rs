use core::prelude::clap_parser::parse_to_naivedate;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Pull,
    Push,
    Clone,
    Account {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<AccountCommands>,
    },
    Partner {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<PartnerCommands>,
    },
    Note {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<NoteCommands>,
    },
    Ledger {
        month: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum AccountCommands {
    All,
    New,
    Remove { id: String },
    SetName { id: String },
}

#[derive(Subcommand)]
pub enum PartnerCommands {
    All,
    New,
    Remove { id: String },
    SetName { id: String },
}

#[derive(Subcommand)]
pub enum NoteCommands {
    New,
    Set {
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        partner: Option<String>,
        #[arg(long, value_parser = parse_to_naivedate)]
        idate: Option<NaiveDate>,
        #[arg(long, value_parser = parse_to_naivedate)]
        cdate: Option<NaiveDate>,
        #[arg(long, value_parser = parse_to_naivedate)]
        ddate: Option<NaiveDate>,
        #[arg(long)]
        net: Option<f32>,
        #[arg(long)]
        vat: Option<f32>,
        #[arg(long)]
        gross: Option<f32>,
    },
    Unset {
        #[arg(long)]
        description: bool,
        #[arg(long)]
        partner: bool,
        #[arg(long)]
        idate: bool,
        #[arg(long)]
        cdate: bool,
        #[arg(long)]
        ddate: bool,
        #[arg(long)]
        net: bool,
        #[arg(long)]
        vat: bool,
        #[arg(long)]
        gross: bool,
    },
    SetTransaction {
        debit: String,
        credit: String,
        amount: String,
    },
}
