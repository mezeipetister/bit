use fs::Inode;

fn main() {
    let mut inode = Inode::default();

    println!(
        "empty inode size is: {}",
        bincode::serialized_size(&inode).unwrap()
    );
}

use bitvec::{bitvec, order::Lsb0, vec::BitVec};

// fn main() {
//     let mut block_bitmap = BitVec::<u8, Lsb0>::with_capacity(4096 as usize);
//     block_bitmap.resize(4096 as usize, false);
//     block_bitmap.set(0, true);
//     block_bitmap.set(3, true);
//     block_bitmap.set(4095, true);
//     let len = block_bitmap.as_bitslice();
//     println!("{}", len);
// }
