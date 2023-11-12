use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    io::{BufReader, Cursor},
    path::Path,
};

fn main() {
    let path = Path::new("demo/demo.db");

    let mut fs = if path.exists() {
        fs::FS::new(path).unwrap()
    } else {
        fs::FS::init(path).unwrap()
    };

    // fs.create_directory("/hello/bello").unwrap();

    // let d = std::fs::File::open("demo/file2.txt").unwrap();
    // let mut data = BufReader::new(&d);

    // fs.add_file(
    //     "/hello/bello",
    //     "demo",
    //     &mut data,
    //     d.metadata().unwrap().len(),
    // )
    // .unwrap();

    let mut d = vec![];
    let mut buf = Cursor::new(&mut d);

    fs.get_file_data("/hello/bello", "demo", &mut buf).unwrap();

    println!("{}", String::from_utf8_lossy(&d));
}
