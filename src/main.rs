use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::hash::Hash;
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
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn get_account_list(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let mut d = self.events.iter().map(|e| e.debit).collect::<Vec<u32>>();
        let mut c = self.events.iter().map(|e| e.credit).collect::<Vec<u32>>();

        result.append(&mut d);
        result.append(&mut c);

        // Dedup result vector
        let set: HashSet<_> = result.drain(..).collect();
        result.extend(set.into_iter());

        result.sort();
        result
    }

    pub fn get_all_account_balance_by_month(&self, date: chrono::NaiveDate) -> HashMap<u32, i32> {
        let mut result: HashMap<u32, i32> = HashMap::new();

        for account in self.get_account_list() {
            let d_sum: u32 = self
                .events
                .iter()
                .filter(|e| e.debit == account)
                .filter(|e| e.performance_date <= date)
                .map(|e| e.value)
                .sum();
            let c_sum: u32 = self
                .events
                .iter()
                .filter(|e| e.credit == account)
                .filter(|e| e.performance_date <= date)
                .map(|e| e.value)
                .sum();

            result.insert(account, d_sum as i32 - c_sum as i32);
        }

        result
    }
}

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
        log.events.push(Event::new(
            items[1].parse::<u32>().unwrap(),
            items[2].parse::<u32>().unwrap(),
            items[3].parse::<u32>().unwrap(),
            get_chrono_naivedate_from_str(&items[0]),
        ));
    }
}

pub fn get_chrono_naivedate_from_str(str: &String) -> chrono::NaiveDate {
    NaiveDate::parse_from_str(&str, "%Y-%m-%d").expect(&format!("Wrong date format"))
}

fn main() {
    let mut log = EventLog { events: Vec::new() };

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

    print_ledger(
        &log.get_all_account_balance_by_month(get_chrono_naivedate_from_str(
            &"2019-04-30".to_string(),
        )),
    );
}

pub fn print_ledger(ledger: &HashMap<u32, i32>) {
    println!("{0: <10} | {1: <10}", "Account", "Balance");
    println!("{0: <10} | {1: <10}", "---", "---");
    for (k, v) in ledger {
        println!("{0: <10} | {1: <10}", k, v);
    }
}
