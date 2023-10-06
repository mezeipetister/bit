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
    Ls,
    Cd { to: String },
    Edit { object_id: String },
    Create { object_id: String },
    Remove { object_id: String },
    Find { filter: String },
    Print { object_id: String },
    Ledger { month: String },
}
