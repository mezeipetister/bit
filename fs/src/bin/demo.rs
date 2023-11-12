use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    io::{BufReader, Cursor},
    path::Path,
};

use fs::{DirectoryIndex, Inode};

fn main() {
    // let p = Path::new("/hello/bello/sub/directory")
    //     .components()
    //     .collect::<Vec>();

    let path = Path::new("demo/demo.db");

    let mut fs = if path.exists() {
        fs::FS::new(path).unwrap()
    } else {
        fs::FS::init(path).unwrap()
    };

    // fs.create_directory("/hello/bello").unwrap();

    // println!("{:?}", fs.superblock);

    // // fs.save_inode(&mut Inode::new(10)).unwrap();

    // let mut inode: Inode = fs.get_inode(10).unwrap();

    // let d = std::fs::File::open("demo/file.txt").unwrap();
    // let mut data = BufReader::new(&d);

    // fs.write_inode_data(&mut inode, &mut data, d.metadata().unwrap().len())
    //     .unwrap();

    // println!("{:?}", inode);

    // let mut d = vec![];
    // let mut buf = Cursor::new(&mut d);
    // let data = fs.read_inode_data(&inode, &mut buf).unwrap();

    // println!("{}", String::from_utf8_lossy(&d));
}
