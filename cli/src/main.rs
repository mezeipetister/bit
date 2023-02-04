use core::{
    index::Db,
    prelude::{CliDisplay, CliError},
};
use std::fmt::Display;

use clap::{Parser, Subcommand};

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
    Account {
        id: Option<String>,
        #[command(subcommand)]
        command: Option<AccountCommands>,
    },
}

#[derive(Subcommand)]
enum AccountCommands {
    All,
    New,
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
                    let mut id = String::new();
                    let mut name = String::new();
                    println!("ID: ");
                    std::io::stdin().read_line(&mut id).unwrap();
                    println!("Name: ");
                    std::io::stdin().read_line(&mut name).unwrap();
                    db.account_add(id.trim().to_string(), name.trim().to_string())?;
                }
                _ => (),
            },
        },

        None => {}
    }
    Ok(())
}
