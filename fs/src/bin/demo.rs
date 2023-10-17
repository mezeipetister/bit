// use fs::Block;

// fn main() {
//     let mut i = Block::Inode {
//         folder: false,
//         created: 0,
//         last_accesed: 0,
//         size: 0,
//         checksum: 0,
//         data: vec![],
//         next: (0, 0),
//     };
//     for _i in 0..4047 {
//         match &mut i {
//             Block::Inode {
//                 folder,
//                 created,
//                 last_accesed,
//                 size,
//                 checksum,
//                 data,
//                 next,
//             } => data.push(0),
//             _ => unimplemented!(),
//         }
//     }
//     println!(
//         "empty inode size is: {}",
//         bincode::serialized_size(&i).unwrap()
//     );
//     println!(
//         "empty vec is: {}",
//         bincode::serialized_size::<Vec<()>>(&vec![]).unwrap()
//     );
// }

fn main() {}
