use core::{
    index::Db,
    prelude::{CliDisplay, CliError},
};
use std::{fmt::Display, ops::Deref};

use clap::{Parser, Subcommand};
use prelude::read_confirm;

use crate::prelude::read_input;

mod prelude;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Pull,
    Push,
    Clone,
    Account {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<AccountCommands>,
    },
    Note {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<NoteCommands>,
    },
}

#[derive(Subcommand)]
enum AccountCommands {
    All,
    New,
    Remove { id: String },
    SetName { id: String },
}

#[derive(Subcommand)]
enum NoteCommands {
    New,
    SetTransaction {
        debit: String,
        credit: String,
        amount: String,
    },
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Init) => {
            let e = Db::init()?;
            println!("Repo inited");
        }
        Some(Commands::Pull) => println!("Pull"),
        Some(Commands::Push) => println!("Push"),
        Some(Commands::Clone) => println!("Clone"),

        Some(Commands::Account { id, command }) => match id {
            Some(id) => {
                let mut db = Db::load()?;
                println!("{}", db.account_get(&id)?)
            }
            None => match command {
                Some(AccountCommands::All) => {
                    let mut db = Db::load()?;
                    println!("{}", db.account_get_all());
                }
                Some(AccountCommands::New) => {
                    let mut db = Db::load()?;
                    let mut id = read_input("ID:");
                    let mut name = read_input("Name:");
                    db.account_add(id.trim().to_string(), name.trim().to_string())?;
                }
                Some(AccountCommands::Remove { id }) => {
                    let mut db = Db::load()?;
                    if read_confirm() {
                        db.account_remove(&id)?;
                    }
                }
                Some(AccountCommands::SetName { id }) => {
                    let mut db = Db::load()?;
                    let name = read_input("New name:");
                    db.account_rename(&id, name)?;
                }
                _ => (),
            },
        },

        Some(Commands::Note { id, command }) => match (id, command) {
            (Some(id), None) => {
                let mut db = Db::load()?;
                println!("{}", db.note_get(&id)?)
            }
            (Some(id), command) => match command {
                Some(NoteCommands::SetTransaction {
                    debit,
                    credit,
                    amount,
                }) => {
                    let mut db = Db::load()?;
                    let debit = db.account_get(&debit)?.to_owned();
                    let credit = db.account_get(&credit)?.to_owned();
                    let mut note = db.note_get_mut(&id)?;
                    note.set_transaction(amount.parse().unwrap(), debit, credit, None)?;
                }
                _ => (),
            },
            (None, command) => match command {
                Some(NoteCommands::New) => {
                    let mut db = Db::load()?;
                    let mut id = read_input("ID:");
                    db.note_new(id.trim().to_string())?;
                }
                _ => (),
            },
            (_, _) => (),
        },

        None => {}
    }
    Ok(())
}
