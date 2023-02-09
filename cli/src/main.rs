use corelib::index::ServerDb;
use corelib::{index::IndexDb, prelude::CliError};
use std::thread::sleep;
use std::time::Duration;

use bit::cli::*;
use bit::prelude::{read_confirm, read_input};
use clap::Parser;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Db { command }) => match command {
            DbCommand::Reindex => {
                let mut db = IndexDb::load()?;
                db.db_reindex()?;
                println!("Ok");
            }
            DbCommand::EmptyIndex => {
                let mut db = IndexDb::load()?;
                db.db_emptyindex()?;
                println!("Ok");
            }
        },
        Some(Commands::Staging) => (),
        Some(Commands::ClearStaging) => (),
        Some(Commands::Log) => (),
        Some(Commands::InitLocal) => {
            let _ = IndexDb::init(repository::sync::Mode::Local)?;
            println!("Repo inited as LOCAL");
        }
        Some(Commands::InitServer { port }) => {
            let _ = IndexDb::init(repository::sync::Mode::Server {
                remote_address: format!("[::1]:{port}"),
            })?;
            println!("Repo inited as SERVER");
        }
        Some(Commands::StartServer) => {
            let db = ServerDb::load().map_err(|e| CliError::Error(e))?;
            db.start_server().map_err(|e| CliError::Error(e))?;
        }
        Some(Commands::Commit { message }) => {
            let mut db = IndexDb::load()?;
            db.commit(message).map_err(|e| CliError::Error(e))?;
            println!("Ok");
        }
        Some(Commands::Pull) => {
            use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
            use std::thread;
            // Provide a custom bar style
            let pb = ProgressBar::new(400);
            pb.set_style(
        ProgressStyle::with_template(
            "Pulling {spinner:.green} [{elapsed_precise}] [{bar:40.white/red}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
            for _ in (0..1000).progress_with(pb) {
                // ...
                thread::sleep(Duration::from_millis(1));
            }

            let mut db = IndexDb::load()?;
            db.pull()?;

            println!("Pull OK");

            let pb = ProgressBar::new(1000);
            pb.set_style(
        ProgressStyle::with_template(
            "Indexing {spinner:.green} [{elapsed_precise}] [{bar:40.white/red}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

            for _ in (0..1000).progress_with(pb) {
                // ...
                thread::sleep(Duration::from_millis(1));
            }

            println!("Indexing OK");
        }
        Some(Commands::Push) => {
            use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
            use std::thread;
            // Provide a custom bar style
            let pb = ProgressBar::new(400);
            pb.set_style(
        ProgressStyle::with_template(
            "Pushing {spinner:.green} [{elapsed_precise}] [{bar:40.white/red}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
            for _ in (0..400).progress_with(pb) {
                thread::sleep(Duration::from_millis(1));
            }

            let mut db = IndexDb::load()?;
            db.push()?;

            println!("Pushed OK");
        }

        Some(Commands::Check) => {
            use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
            use std::thread;
            // Provide a custom bar style
            let pb = ProgressBar::new(400);
            pb.set_style(
        ProgressStyle::with_template(
            "Checking {spinner:.green} [{elapsed_precise}] [{bar:40.white/red}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
            for _ in (0..400).progress_with(pb) {
                thread::sleep(Duration::from_millis(1));
            }

            println!("All OK");
        }

        Some(Commands::Clone { remote_address }) => {
            let db = IndexDb::init(repository::sync::Mode::Server { remote_address })?;
        }

        Some(Commands::Account { id, command }) => match (id, command) {
            (Some(id), None) => {
                let db = IndexDb::load()?;
                println!("{}", db.account_get(&id)?)
            }
            (Some(id), Some(command)) => match command {
                AccountCommands::Remove { y } => {
                    let mut db = IndexDb::load()?;
                    if read_confirm(y) {
                        db.account_remove(&id)?;
                    }
                }
                AccountCommands::Restore => {
                    let mut db = IndexDb::load()?;
                    db.account_restore(&id)?;
                }
                AccountCommands::History => {
                    let db = IndexDb::load()?;
                    let res = db.account_history(&id)?;
                    println!("{res}");
                }
                AccountCommands::Set { name } => {
                    let mut db = IndexDb::load()?;
                    let name = name.unwrap_or_else(|| read_input("New name:"));
                    db.account_rename(&id, name)?;
                }
                _ => println!("Unknown command"),
            },
            (None, Some(command)) => match command {
                AccountCommands::All => {
                    let mut db = IndexDb::load()?;
                    println!("{}", db.account_get_all());
                }
                AccountCommands::Add { id, name } => {
                    let mut db = IndexDb::load()?;
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
                let db = IndexDb::load()?;
                println!("{}", db.partner_get(&id)?)
            }
            (Some(id), Some(command)) => match command {
                PartnerCommands::Remove { y } => {
                    let mut db = IndexDb::load()?;
                    if read_confirm(y) {
                        db.partner_remove(&id)?;
                    }
                }
                PartnerCommands::Set { name } => {
                    let mut db = IndexDb::load()?;
                    let name = name.unwrap_or_else(|| read_input("New name:"));
                    db.partner_rename(&id, name)?;
                }
                _ => (),
            },
            (None, Some(command)) => match command {
                PartnerCommands::All => {
                    let db = IndexDb::load()?;
                    println!("{}", db.partner_get_all());
                }
                PartnerCommands::Add { id, name } => {
                    let mut db = IndexDb::load()?;
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
                let db = IndexDb::load()?;
                println!("{}", db.note_get(&id)?)
            }
            (Some(id), command) => match command {
                Some(NoteCommands::SetTransaction {
                    debit,
                    credit,
                    amount,
                    comment,
                }) => {
                    let mut db = IndexDb::load()?;
                    let debit = db.account_get(&debit)?.to_owned();
                    let credit = db.account_get(&credit)?.to_owned();
                    db.note_set_transaction(&id, debit, credit, amount.parse().unwrap(), comment)?;
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
                    let mut db = IndexDb::load()?;
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
                    let mut db = IndexDb::load()?;
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
                    let mut db = IndexDb::load()?;
                    let id = id.unwrap_or_else(|| read_input("ID:"));
                    db.note_add(id.trim().to_string())?;
                }
                Some(NoteCommands::Filter { id, partner }) => {
                    let db = IndexDb::load()?;
                    let res = db.note_filter(id, partner);
                    res.iter().for_each(|res| println!("{res}"));
                }
                _ => (),
            },
        },

        Some(Commands::Ledger { month }) => {
            let mut db = IndexDb::load()?;
            let res = db.get_ledger(month)?;
            println!("{res}");
        }

        None => {}
    }
    Ok(())
}
