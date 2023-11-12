use fs::FS;
use std::{
    io::{BufReader, Cursor},
    path::Path,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds files to myapp
    Add {
        from: String,
        path: String,
        filename: String,
    },
    Get {
        path: String,
        filename: String,
    },
    Info,
}

fn main() {
    let path = Path::new("demo/demo.db");

    let mut fs = if path.exists() {
        fs::FS::new(path).unwrap()
    } else {
        fs::FS::init(path).unwrap()
    };

    let cli = Cli::parse();

    match cli.command {
        Commands::Info => println!("{:?}", &fs.superblock),
        Commands::Add {
            from,
            path,
            filename,
        } => {
            add_file(&mut fs, &from, &path, &filename);
        }
        Commands::Get { path, filename } => {
            print_file(&mut fs, &path, &filename);
        }
    }
}

fn add_file(fs: &mut FS, file_path: &str, path: &str, file_name: &str) {
    fs.create_directory(path).unwrap();

    let d = std::fs::File::open(file_path).unwrap();
    let mut data = BufReader::new(&d);

    fs.add_file(path, file_name, &mut data, d.metadata().unwrap().len())
        .unwrap();
}

fn print_file(fs: &mut FS, path: &str, file_name: &str) {
    let mut d = vec![];
    let mut buf = Cursor::new(&mut d);

    fs.get_file_data(path, file_name, &mut buf).unwrap();

    println!("{}", String::from_utf8_lossy(&d));
}
