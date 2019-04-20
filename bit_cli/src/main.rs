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
extern crate structopt;
mod parser;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
// use chrono::prelude::*;

/// Read file to string
/// Gets a File reference, tries to read it, once its done returns a Result
pub fn read_file_to_string(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// pub fn process_file(file: &String, log: &mut EventLog) {
//     // Collect not comment lines here!
//     let mut i = 0;
//     for line in file.split("\n") {
//         // Let's check the next line
//         i = i + 1;

//         let line_chars: Vec<char> = line.chars().collect();

//         if line_chars.len() > 1 {
//             // Check if this line is at least 1 char long, otherwise its empty
//             if line_chars[0] as u8 == b'/' {
//                 // if first character is /
//                 if line_chars[1] as u8 == b'/' {
//                     // if second character is /
//                     continue; // Miss this line as its a comment line
//                 }
//             }
//         } else {
//             continue; // Miss this line as its an empty line;
//         }

//         // Ok, so now we have a clean line to process
//         // ..
//         // ..
//         let items: Vec<String> = line
//             .split("\t")
//             .filter(|&e| e.len() > 0)
//             .map(|s| s.to_string())
//             .collect();

//         if items.len() != 4 {
//             panic!("Error at line {line}: Wrong item number!", line = i);
//         }

//         // Process each row
//         // Create a new event
//         // And push it to the log
//         log.events.push(Event::new(
//             items[1].parse::<u32>().unwrap(),
//             items[2].parse::<u32>().unwrap(),
//             items[3].parse::<u32>().unwrap(),
//             get_chrono_naivedate_from_str(&items[0]),
//         ));
//     }
// }

// pub fn get_chrono_naivedate_from_str(str: &String) -> chrono::NaiveDate {
//     NaiveDate::parse_from_str(&str, "%Y-%m-%d").expect(&format!("Wrong date format"))
// }

fn main() {
    // let files = fs::read_dir(env::current_dir().expect("Error while determining current dir"))
    //     .expect("Error during reading folder..")
    //     .filter_map(|entry| {
    //         entry.ok().and_then(|e| {
    //             e.path()
    //                 .file_name()
    //                 .and_then(|n| n.to_str().map(|s| String::from(s)))
    //         })
    //     })
    //     .collect::<Vec<String>>();

    // let mut raw_content: Vec<String> = Vec::new();

    // for file in files {
    //     if Path::new(&file).extension().and_then(OsStr::to_str) != Some("bit") {
    //         continue;
    //     }

    //     let mut _file = File::open(&file).expect(&format!("Error while opening file: {}", file));
    //     raw_content
    //         .push(read_file_to_string(&mut _file).expect("Error while reading file to string"));
    // }

    // for content in raw_content {
    //     process_file(&content, &mut log);
    // }
}
