use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    debit: u32,
    credit: u32,
    value: u32,
    performance_date: chrono::NaiveDate,
}

impl Event {
    // Get a new event to work with!
    pub fn new(debit: u32, credit: u32, value: u32, performance_date: chrono::NaiveDate) -> Self {
        Event {
            debit,
            credit,
            value,
            performance_date,
        }
    }
}

// This will contain all the events
pub type EventLog = Vec<Event>;

/// Read file to string
/// Gets a File reference, tries to read it, once its done returns a Result
pub fn read_file_to_string(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn process_file(file: &String, log: &mut EventLog) {
    // Collect not comment lines here!
    let mut i = 0;
    for line in file.split("\n") {
        // Let's check the next line
        i = i + 1;

        let line_chars: Vec<char> = line.chars().collect();

        if line_chars.len() > 1 {
            // Check if this line is at least 1 char long, otherwise its empty
            if line_chars[0] as u8 == b'/' {
                // if first character is /
                if line_chars[1] as u8 == b'/' {
                    // if second character is /
                    continue; // Miss this line as its a comment line
                }
            }
        } else {
            continue; // Miss this line as its an empty line;
        }

        // Ok, so now we have a clean line to process
        // ..
        // ..
        let items: Vec<String> = line
            .split("\t")
            .filter(|&e| e.len() > 0)
            .map(|s| s.to_string())
            .collect();

        if items.len() != 4 {
            panic!("Error at line {line}: Wrong item number!", line = i);
        }

        // Process each row
        // Create a new event
        // And push it to the log
        log.push(Event::new(
            items[1].parse::<u32>().unwrap(),
            items[2].parse::<u32>().unwrap(),
            items[3].parse::<u32>().unwrap(),
            NaiveDate::parse_from_str(&items[0], "%Y.%m.%d")
                .expect(&format!("Wrong date format at line {line}", line = i)),
        ));
    }
}

fn main() {
    let mut log = EventLog::new();

    let files = fs::read_dir(env::current_dir().expect("Error while determining current dir"))
        .expect("Error during reading folder..")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    let mut raw_content: Vec<String> = Vec::new();

    for file in files {
        if Path::new(&file).extension().and_then(OsStr::to_str) != Some("bit") {
            continue;
        }

        let mut _file = File::open(&file).expect(&format!("Error while opening file: {}", file));
        raw_content
            .push(read_file_to_string(&mut _file).expect("Error while reading file to string"));
    }

    for content in raw_content {
        process_file(&content, &mut log);
    }

    println!("{:?}", log);
}
