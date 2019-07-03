/**
 * BIT
 * Copyright (C) 2019 Peter Mezei <mezeipetister@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 */
extern crate bit_core;
extern crate num_format;
extern crate open;

mod jsonparser;
mod parser;

use bit_core::account::*;
use bit_core::event::*;
use chrono::prelude::*;
use chrono::{Duration, TimeZone, Utc};
use num_format::{Locale, ToFormattedString};
// use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), String> {
    // File reader
    let read_file = |file: &mut File| -> Result<String, io::Error> {
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => return Ok(content),
            Err(msg) => return Err(msg),
        }
    };

    // Let's define current working directory
    // As we use env::current_dir(), this directory
    // will be one where BIT command was hit.
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(msg) => {
            return Err(format!("{}", msg));
        }
    };

    // Read account file
    // Check errors
    // TODO: Refact! This is too complicated for one function
    let accounts = match File::open(current_dir.join("accounts")) {
        Ok(mut file) => match read_file(&mut file) {
            Ok(content) => {
                // Here we collect the valid accounts to return
                let mut result: Vec<Account> = Vec::new();
                // Index counter for line reading
                // This will help to print the line number
                // where error found
                let mut index = 0;
                // Split content by new lines
                for line in content.split("\n") {
                    index = index + 1;
                    let line_array = match parser::clean_line(line.trim()) {
                        // Trim the line!
                        Some(line) => line,
                        None => continue, // Skip empty lines
                    };
                    // If we have more field then 2 return error.
                    if line_array.len() != 2 {
                        return Err(format!(
                            "We have more or less field(s) then 2 at line {}",
                            index
                        ));
                    }
                    match add_account(&mut result, new_account(line_array[0], line_array[1])) {
                        Ok(_) => (),
                        Err(msg) => return Err(format!("{}", msg)),
                    }
                }
                result
            }
            Err(msg) => {
                return Err(format!("{}", msg));
            }
        },
        Err(_) => {
            return Err(format!("Error while reading account file"));
        }
    };

    // Read events
    // TODO: Refact it!
    let found_files: Vec<String> = match fs::read_dir(&current_dir) {
        Ok(files) => files
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>(),
        Err(_) => return Err(format!("Error while reading path for .bit files")),
    };

    // Store events
    let mut events: Vec<Event> = Vec::new();

    for file in found_files {
        // Process only .bit files
        if Path::new(&file).extension().and_then(OsStr::to_str) != Some("bit") {
            continue;
        }

        // Open bit file to read
        match File::open(current_dir.join(file)) {
            // Read file
            Ok(mut file) => match read_file(&mut file) {
                Ok(content) => {
                    // Index counter for line reading
                    // This will help to print the line number
                    // where error found
                    let mut index = 0;
                    // Split content by new lines
                    for line in content.split("\n") {
                        index = index + 1;
                        let line_array = match parser::clean_line(&mut line.trim()) {
                            // Trim the line!
                            Some(line) => line,
                            None => continue, // Skip empty lines
                        };
                        // If we have more field then 4 return error.
                        if line_array.len() != 4 {
                            return Err(format!(
                                "We have more or less field(s) then 4 at line {}",
                                index
                            ));
                        }

                        // Try to parse value to u32
                        let value: u32 = match line_array[3].parse::<u32>() {
                            Ok(result) => result,
                            Err(msg) => {
                                return Err(format!(
                                    "Error during value parse at line {}. {}",
                                    index, msg
                                ));
                            }
                        };

                        // Try to parse date to NaiveDate (%Y-%m-%d)
                        let date: NaiveDate =
                            match NaiveDate::parse_from_str(line_array[0], "%Y-%m-%d") {
                                Ok(date) => date,
                                Err(msg) => {
                                    return Err(format!(
                                        "Error during date parse at line {}. {}",
                                        index, msg
                                    ));
                                }
                            };

                        // Try read debit
                        let debit: &str = line_array[1].trim();

                        // Validate debit
                        if !is_valid_account(&accounts, debit) {
                            return Err(format!("Not valid debit account ID at line {}", index));
                        }

                        // Debit is leaf?
                        if !check_account_is_leaf(&accounts, debit) {
                            return Err(format!(
                                "Debit account ID ({}) is not leaf! Use leaf instead! at line {}",
                                debit, index
                            ));
                        }

                        // Try read credit
                        let credit: &str = line_array[2].trim();

                        // Validate credit
                        if !is_valid_account(&accounts, credit) {
                            return Err(format!("Not valid credit account ID at line {}", index));
                        }

                        // Credit is leaf?
                        if !check_account_is_leaf(&accounts, credit) {
                            return Err(format!(
                                "Credit account ID ({}) is not leaf! Use leaf instead! at line {}",
                                credit, index
                            ));
                        }

                        add_event(&mut events, new_event(debit, credit, value, date));
                    }
                }
                Err(msg) => {
                    return Err(format!("{}", msg));
                }
            },
            Err(_) => {
                return Err(format!("Error while reading .bit file"));
            }
        };
    }

    // Read CLI arguments
    let args: Vec<String> = env::args().collect();

    for arg in &args {
        // Create report
        if arg == "report" {
            let now = Local::now();
            let file_name = format!(
                "{}_{}_{}_{}_{}_{}",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            let mut create_report = Command::new("R");
            create_report.arg("-e").arg(format!(
                r#"rmarkdown::render('report.rmd', output_file='./report/{}.html')"#,
                file_name
            ));
            let _result = create_report
                .output()
                .expect("failed to create report output!");
            open::that(
                current_dir
                    .join("report")
                    .join(format!("{}.html", file_name)),
            )
            .unwrap();
            println!("Report done!");
            return Ok(());
        }
    }

    // Define today
    let dt = Local::now();
    let mut date = NaiveDate::from_ymd(dt.year(), dt.month(), dt.day());

    // If we have provided date as filter, then apply it
    if args.len() == 2 || args.len() == 3 {
        date = match NaiveDate::parse_from_str(&args[1], "%Y-%m-%d") {
            Ok(date) => date,
            Err(msg) => {
                return Err(format!(
                    "Error with the given date parameter during call. {}",
                    msg
                ));
            }
        };
    }

    // Refact needed!
    // Its just for testing purpose!
    for arg in args {
        if arg == "csv" {
            // Create the csv writer buffer
            let mut wtr = csv::Writer::from_writer(io::stdout());

            // Write header
            let mut header: Vec<String> = Vec::new();
            header.push("Date".to_string());
            for account in accounts.clone() {
                let aid = account.id;
                header.push(aid.clone());
            }
            wtr.write_record(&header).unwrap();

            // Define January 1.
            let mut current_date = Utc.ymd(dt.year(), 1, 1);
            let till_date = Utc.ymd(date.year(), date.month(), date.day());

            // Iterate over the year, day by day
            while current_date <= till_date {
                let mut row = Vec::new();
                row.push(format!(
                    "{}-{}-{}",
                    current_date.year(),
                    current_date.month(),
                    current_date.day(),
                ));
                for account in accounts.clone() {
                    row.push(
                        get_ledger_by_account_id_and_by_date(
                            &account.id,
                            &events,
                            current_date.naive_utc(),
                        )
                        .to_string(),
                    );
                }
                wtr.write_record(&row).unwrap();
                current_date = current_date + Duration::days(1);
            }

            // Flush csv writer io buffer
            wtr.flush().unwrap();

            // for account in accounts {
            //     result.insert(
            //         account.id.clone(),
            //         get_ledger_by_account_id_and_by_date(&account.id, &events, date),
            //     );
            // }
            // println!("{}", jsonparser::to_string(result));
            return Ok(());
        }
    }

    println!("{0: <4} {1: <15} | {2: <10}", "ID", "Name", "Balance");

    // Print ledger as result
    for account in accounts {
        let mut name: String = "".to_string();
        let mut index: u32 = 0;
        for char in account.name.chars() {
            if index > 13 {
                continue;
            }
            index = index + 1;
            name.push(char);
        }

        println!(
            "{0: <4} {1: <15} | {2: <10}",
            account.id,
            name,
            get_ledger_by_account_id_and_by_date(&account.id, &events, date)
                .to_formatted_string(&Locale::en)
        )
    }

    Ok(())
}
