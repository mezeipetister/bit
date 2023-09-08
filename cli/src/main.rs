use corelib::index::ServerDb;
use corelib::{index::IndexDb, prelude::CliError};

use std::time::Duration;

use bit::cli::*;
use bit::prelude::{read_confirm, read_input};
use clap::Parser;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Ls) => println!("List"),
        Some(Commands::Init { name }) => println!("Init {}", name),
        Some(Commands::Open { name }) => println!("Open {}", name),
        Some(Commands::Remove { name }) => println!("Remove {}", name),
        None => (),
    }
    Ok(())
}
