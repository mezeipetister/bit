use core::{index::Db, prelude::CliError};

use bit::cli::*;
use bit::prelude::{read_confirm, read_input};
use clap::Parser;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    let sudo = cli.y;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Init) => {
            let _ = Db::init()?;
            println!("Repo inited");
        }
        Some(Commands::Pull) => println!("Pull"),
        Some(Commands::Push) => println!("Push"),
        Some(Commands::Clone) => println!("Clone"),

        Some(Commands::Account { id, command }) => match (id, command) {
            (Some(id), None) => {
                let db = Db::load()?;
                println!("{}", db.account_get(&id)?)
            }
            (Some(id), Some(command)) => match command {
                AccountCommands::Remove => {
                    let mut db = Db::load()?;
                    if read_confirm(sudo) {
                        db.account_remove(&id)?;
                    }
                }
                AccountCommands::Set { name } => {
                    let mut db = Db::load()?;
                    let name = name.unwrap_or_else(|| read_input("New name:"));
                    db.account_rename(&id, name)?;
                }
                _ => println!("Unknown command"),
            },
            (None, Some(command)) => match command {
                AccountCommands::All => {
                    let db = Db::load()?;
                    println!("{}", db.account_get_all());
                }
                AccountCommands::Add { id, name } => {
                    let mut db = Db::load()?;
                    let id = id.unwrap_or_else(|| read_input("ID:"));
                    let name = name.unwrap_or_else(|| read_input("Name:"));
                    db.account_add(id.trim().to_string(), name.trim().to_string())?;
                }
                _ => (),
            },
            _ => (),
        },

        Some(Commands::Partner { id, command }) => match (id, command) {
            (Some(id), None) => {
                let db = Db::load()?;
                println!("{}", db.partner_get(&id)?)
            }
            (Some(id), Some(command)) => match command {
                PartnerCommands::Remove => {
                    let mut db = Db::load()?;
                    if read_confirm(sudo) {
                        db.partner_remove(&id)?;
                    }
                }
                PartnerCommands::Set { name } => {
                    let mut db = Db::load()?;
                    let name = name.unwrap_or_else(|| read_input("New name:"));
                    db.partner_rename(&id, name)?;
                }
                _ => (),
            },
            (None, Some(command)) => match command {
                PartnerCommands::All => {
                    let db = Db::load()?;
                    println!("{}", db.partner_get_all());
                }
                PartnerCommands::Add { id, name } => {
                    let mut db = Db::load()?;
                    let id = id.unwrap_or_else(|| read_input("ID:"));
                    let name = name.unwrap_or_else(|| read_input("Name:"));
                    db.partner_add(id.trim().to_string(), name.trim().to_string())?;
                }
                _ => (),
            },
            _ => (),
        },

        Some(Commands::Note { id, command }) => match (id, command) {
            (Some(id), None) => {
                let db = Db::load()?;
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
                    let note = db.note_get_mut(&id)?;
                    note.set_transaction(amount.parse().unwrap(), debit, credit, None)?;
                }
                Some(NoteCommands::Set {
                    description,
                    partner,
                    idate,
                    cdate,
                    ddate,
                    net,
                    vat,
                    gross,
                }) => {
                    let mut db = Db::load()?;
                    db.ledger_set_should_update();
                    let partner = match partner {
                        Some(id) => {
                            let partner = db.partner_get(&id)?.to_owned();
                            Some(partner)
                        }
                        None => None,
                    };
                    db.note_set(
                        &id,
                        partner,
                        description,
                        idate,
                        cdate,
                        ddate,
                        net,
                        vat,
                        gross,
                    )?;
                }
                Some(NoteCommands::Unset {
                    description,
                    partner,
                    idate,
                    cdate,
                    ddate,
                    net,
                    vat,
                    gross,
                }) => {
                    let mut db = Db::load()?;
                    db.ledger_set_should_update();
                    db.note_unset(
                        &id,
                        partner,
                        description,
                        idate,
                        cdate,
                        ddate,
                        net,
                        vat,
                        gross,
                    )?;
                }
                _ => (),
            },
            (None, command) => match command {
                Some(NoteCommands::Add { id }) => {
                    let mut db = Db::load()?;
                    let id = id.unwrap_or_else(|| read_input("ID:"));
                    db.note_new(id.trim().to_string())?;
                }
                Some(NoteCommands::Filter { id, partner }) => {
                    let db = Db::load()?;
                    let res = db.note_filter(id, partner);
                    res.iter().for_each(|res| println!("{res}"));
                }
                _ => (),
            },
        },

        Some(Commands::Ledger { month }) => {
            let mut db = Db::load()?;
            let res = db.get_ledger(month)?;
            println!("{res}");
        }

        None => {}
    }
    Ok(())
}
