use core::prelude::clap_parser::*;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    #[arg(short, long, action, global = true)]
    pub y: bool,
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
        #[arg(value_parser = parse_month)]
        month: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum AccountCommands {
    All,
    New {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },
    Remove {
        id: String,
    },
    SetName {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PartnerCommands {
    All,
    New {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },
    Remove {
        id: String,
    },
    SetName {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum NoteCommands {
    New {
        #[arg(long)]
        id: Option<String>,
    },
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
        #[arg(short, long)]
        debit: String,
        #[arg(short, long)]
        credit: String,
        #[arg(short, long)]
        amount: String,
    },
}
