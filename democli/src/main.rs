use std::{
    io::{BufRead, BufReader, Cursor, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use cli::{terminal::Terminal, Cli};
use editor::{Document, Editor};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use walnut::FS;

const SECRET: &'static str = "hellobello";

fn init_db() -> FS {
    let path = Path::new("/home/petermezei/Downloads/gfsdemo/demodemo.db");
    if path.exists() {
        FS::new(path, SECRET).unwrap()
    } else {
        FS::init(path, SECRET).unwrap()
    }
}

fn parse_line(line: &str) -> (String, String) {
    let parts: Vec<&'_ str> = line.split(" ").collect();
    (parts[0].to_string(), parts[1..].join(" "))
}

fn set_file(fs: &mut FS, path: String, file_name: String, content: &[u8]) {
    // let mut fs = fs.lock().unwrap();
    fs.create_directory(&path).unwrap();

    let mut data = BufReader::new(content);

    fs.add_file(&path, &file_name, &mut data, content.len() as u64)
        .unwrap();
}

fn get_file(fs: &mut FS, path: &str, file_name: &str) -> Result<String, String> {
    // let mut fs = fs.lock().unwrap();
    let mut d = vec![];
    let mut buf = Cursor::new(&mut d);

    fs.get_file_data(path, file_name, &mut buf)
        .map_err(|_| "No file found".to_string())?;

    Ok(String::from_utf8_lossy(&d).to_string())
}

fn get_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PDF File", &["pdf"])
        .show_open_single_file()
        .unwrap()
}

fn main() {
    let mut fs = init_db();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // loop {
    //     stdout.flush().unwrap();
    //     let line = stdin.lock().lines().next().unwrap().unwrap();
    // }

    let actions = |line: String| -> Result<(), String> {
        println!("");
        let parsed = parse_line(&line);

        match parsed.0.as_str() {
            "hello" => println!("Bello!"),
            "select" => {
                println!("{:?}", get_path());
            }
            "clear" => {
                Terminal::clear_screen();
                Terminal::goto(0, 0);
            }
            "touch" => {
                let parts: Vec<&'_ str> = parsed.1.split(" ").collect();
                let path = parts[0].to_string();
                let filename = parts[1].to_string();
                match fs.add_file(&path, &filename, &mut BufReader::new(Cursor::new([])), 0) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error creating file");
                    }
                }
            }
            "mkdir" => {
                let parts: Vec<&'_ str> = parsed.1.split(" ").collect();
                let path = parts[0].to_string();
                match fs.create_directory(&path) {
                    Ok(_) => println!("Created"),
                    Err(_) => {
                        println!("Error creating directory");
                    }
                }
            }
            "open" => {
                let parts: Vec<&'_ str> = parsed.1.split(" ").collect();
                let path = parts[0].to_string();
                let filename = parts[1].to_string();
                // Open file
                let content = match get_file(&mut fs, &path, &filename) {
                    Ok(content) => content,
                    Err(_) => {
                        println!("No file found!");
                        return Ok(());
                    }
                };

                let on_save = |content: String| -> Result<(), String> {
                    set_file(&mut fs, path.clone(), filename.clone(), content.as_bytes());
                    println!("Saved");
                    Ok(())
                };

                let document = Document::new(parts[1].to_string(), content, Box::new(on_save));

                let mut editor = Editor::new(document, &stdin, &stdout).unwrap();

                editor.run().unwrap();

                drop(editor);
            }
            "quit" => {
                println!("bye!");
                return Ok(());
            }
            _ => println!("Unknown command"),
        }
        Ok(())
    };

    let mut cli = Cli::new(&stdout, &stdin, actions).unwrap();
    cli.run().unwrap();
}
