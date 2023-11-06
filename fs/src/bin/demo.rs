use std::path::Path;

use fs::Inode;

fn main() {
    let path = Path::new("demo.db");

    let mut fs = if path.exists() {
        fs::FS::new(path).unwrap()
    } else {
        fs::FS::init(path, 1).unwrap()
    };

    fs.save_inode(&mut Inode::default()).unwrap();
}

// fn main() {
//     let mut block_bitmap = BitVec::<u8, Lsb0>::with_capacity(4096 as usize);
//     block_bitmap.resize(4096 as usize, false);
//     block_bitmap.set(0, true);
//     block_bitmap.set(3, true);
//     block_bitmap.set(4095, true);
//     let len = block_bitmap.as_bitslice();
//     println!("{}", len);
// }
