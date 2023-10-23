use anyhow::anyhow;
use bitvec::{order::Lsb0, vec::BitVec};
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Seek, SeekFrom};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    io::{Read, Write},
    path::Path,
    time::{self, SystemTime},
};
// const M: u32 = 0xb1a9;
const MAGIC: [u8; 7] = *b"*bitfs*";
const ROOT_INODE: u32 = 1;
const BLOCK_SIZE: u32 = 4096;
const BLOCKS_PER_GROUP: u32 = BLOCK_SIZE * 8;

#[inline]
pub fn calculate_checksum<S>(s: &S) -> u32
where
    S: serde::Serialize,
{
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&bincode::serialize(&s).unwrap());
    hasher.finalize()
}

#[inline]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, Default)]
pub struct FS {
    pub superblock: Option<Superblock>,
    pub mmap: Option<MmapMut>,
    pub groups: Option<Vec<Group>>,
}

impl FS {
    // pub fn new<P>(image_path: P) -> anyhow::Result<Self>
    // where
    //     P: AsRef<Path>,
    // {
    //     let file = OpenOptions::new()
    //         .read(true)
    //         .write(true)
    //         .open(image_path.as_ref())?;
    //     let mmap = unsafe { MmapMut::map_mut(&file)? };
    //     let mut cursor = Cursor::new(&mmap);
    //     let sb: Superblock = Superblock::deserialize_from(&mut cursor)?;
    //     let groups = Group::deserialize_from(&mut cursor, sb.block_size, sb.groups as usize)?;

    //     let mut fs = Self {
    //         sb: Some(sb),
    //         groups: Some(groups),
    //         mmap: Some(mmap),
    //     };

    //     fs.create_root()?;

    //     Ok(fs)
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Superblock {
    magic: [u8; 7],        // Magic number to check
    block_size: u32,       // Block size in bytes
    blocks_per_group: u32, // max data block per group
    total_groups: u32,     // Total groups count
    total_blocks: u32,     // Total blocks count
    allocated_blocks: u32, // Allocated blocks
    free_blocks: u32,      // Available blocks
    total_folders: u32,    // Folder count in fs
    total_files: u32,      // File count in fs
    created: u64,          // FS creation time
    modified: u64,         // FS last modification time
    checksum: u32,         // Superblock checksum
}

impl Superblock {
    fn new() -> Self {
        Self {
            magic: MAGIC,
            block_size: BLOCK_SIZE,
            blocks_per_group: BLOCKS_PER_GROUP,
            total_groups: 0,
            total_blocks: 1,
            allocated_blocks: 1,
            free_blocks: 0,
            total_folders: 0,
            total_files: 0,
            created: now(),
            modified: now(),
            checksum: 0,
        }
    }

    pub fn update_modified(&mut self) {
        self.modified = now();
    }

    #[allow(dead_code)]
    pub fn serialize(&mut self) -> anyhow::Result<Vec<u8>> {
        self.checksum();
        bincode::serialize(self).map_err(|e| e.into())
    }

    pub fn serialize_into<W>(&mut self, w: W) -> anyhow::Result<()>
    where
        W: Write,
    {
        self.checksum();
        bincode::serialize_into(w, self).map_err(|e| e.into())
    }

    pub fn deserialize_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut sb: Self = bincode::deserialize_from(r)?;
        if !sb.verify_checksum() {
            return Err(anyhow!("Superblock checksum verification failed"));
        }

        Ok(sb)
    }

    fn checksum(&mut self) {
        self.checksum = 0;
        self.checksum = calculate_checksum(&self);
    }

    fn verify_checksum(&mut self) -> bool {
        let checksum = self.checksum;
        self.checksum = 0;
        let ok = checksum == calculate_checksum(&self);
        self.checksum = checksum;

        ok
    }
}

#[derive(Debug, Default)]
pub struct Group {
    pub block_bitmap: BitVec<u8, Lsb0>,
    next_block: Option<usize>,
}

impl Group {
    fn new(id: u32, block_bitmap: BitVec<u8, Lsb0>) -> Self {
        let mut group = Self {
            block_bitmap,
            next_block: None,
        };
        group.next_block = group.next_free_data_block();
        group
    }

    fn seek_position(id: u32) -> u32 {
        // Superblock BLOCK_SIZE (4kib)
        // + Group ID * (BLOCK_SIZE + BLOCKS_PER_GROUP * BLOCK_SIZE)
        BLOCK_SIZE + id * (BLOCK_SIZE + BLOCKS_PER_GROUP * BLOCK_SIZE)
    }

    fn bitmap_seek_position(&self, id: u32) -> u32 {
        Self::seek_position(id)
    }

    pub fn serialize_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: Write + Seek,
    {
        w.write_all(self.block_bitmap.as_raw_slice())?;

        Ok(())
    }

    pub fn deserialize_from<R>(mut r: R, id: u32) -> anyhow::Result<Group>
    where
        R: Read + Seek,
    {
        let mut buf = Vec::with_capacity(BLOCK_SIZE as usize);
        unsafe {
            buf.set_len(BLOCK_SIZE as usize);
        }

        let offset = Self::seek_position(id);
        r.seek(SeekFrom::Start(offset as u64))?;
        r.read_exact(&mut buf)?;
        let data_bitmap = BitVec::<u8, Lsb0>::from_slice(&buf);

        Ok(Group::new(id, data_bitmap))
    }

    #[inline]
    pub fn has_data_block(&self, i: usize) -> bool {
        self.block_bitmap.get(i - 1).as_deref().unwrap_or(&false) == &true
    }

    #[inline]
    pub fn free_data_blocks(&self) -> usize {
        self.block_bitmap.count_zeros()
    }

    #[inline]
    pub fn total_data_blocks(&self) -> usize {
        self.block_bitmap.len()
    }

    #[inline]
    pub fn allocate_data_block(&mut self) -> Option<usize> {
        self.next_block.and_then(|index| {
            self.add_data_block(index);
            self.next_block = self.next_free_data_block();
            Some(index)
        })
    }

    #[inline]
    pub fn release_data_block(&mut self, index: usize) {
        self.block_bitmap.set(index - 1, false);
        self.next_block = self.next_free_data_block();
    }

    #[inline]
    fn add_data_block(&mut self, i: usize) {
        self.block_bitmap.set(i - 1, true);
    }

    #[inline]
    fn next_free_data_block(&self) -> Option<usize> {
        self.block_bitmap
            .iter()
            .position(|bit| !*bit)
            .map(|p| p + 1)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inode {
    id: u32,
    folder: bool,
    created: u64,
    last_accesed: u64,
    size: u64,
    checksum: u32,
    block_count: u32,
    data: Vec<u8>,
    next: (u32, u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    inode_id: u32,
    position: u32,
    data: Vec<u8>,
    next: (u32, u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Block {
    Empty,
    Inode(Inode),
    Chunk(Chunk),
}

impl Block {
    pub fn is_empty(&self) -> bool {
        match self {
            Block::Empty => true,
            _ => false,
        }
    }

    pub fn as_inode(self) -> anyhow::Result<Inode> {
        match self {
            Block::Inode(inode) => Ok(inode),
            _ => Err(anyhow!("Not an inode!")),
        }
    }

    pub fn as_data(self) -> anyhow::Result<Chunk> {
        match self {
            Block::Chunk(data) => Ok(data),
            _ => Err(anyhow!("Not a data")),
        }
    }

    pub fn serialize_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: Write + Seek,
    {
        bincode::serialize_into(w, self).map_err(|e| e.into())
    }

    pub fn deserialize_from<R>(r: R) -> anyhow::Result<Block>
    where
        R: Read + Seek,
    {
        bincode::deserialize_from(r).map_err(|_| anyhow!("Block deser error. Not a valid block"))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Directory {
    pub directories: BTreeMap<OsString, u32>,
    checksum: u32,
}

impl Directory {
    pub fn serialize_into<W>(&mut self, w: W) -> anyhow::Result<()>
    where
        W: Write,
    {
        self.checksum();
        bincode::serialize_into(w, self).map_err(|e| e.into())
    }

    pub fn deserialize_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut sb: Self = bincode::deserialize_from(r)?;
        if !sb.verify_checksum() {
            return Err(anyhow!("Directory checksum verification failed"));
        }

        Ok(sb)
    }

    pub fn entry<P>(&self, path: P) -> Option<u32>
    where
        P: AsRef<Path>,
    {
        self.directories
            .get(&path.as_ref().as_os_str().to_os_string())
            .map(|x| *x)
    }

    fn checksum(&mut self) {
        self.checksum = 0;
        self.checksum = calculate_checksum(&self);
    }

    fn verify_checksum(&mut self) -> bool {
        let checksum = self.checksum;
        self.checksum = 0;
        let ok = checksum == calculate_checksum(&self);
        self.checksum = checksum;

        ok
    }
}

pub fn allocator(data: &[u8]) -> Vec<(u32, u32)> {
    let x = data.len();
    let block_needed = (x / BLOCK_SIZE as usize + usize::from(x % BLOCK_SIZE as usize != 0)) as u32;
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::time::{self, SystemTime};

    #[test]
    fn test_block_bitmap_seek_position() {
        // let group = Group::new(0);
        // assert_eq!(group.bitmap_seek_position(), BLOCK_SIZE);

        // let group = Group::new(1);
        // assert_eq!(group.bitmap_seek_position(), 134_221_824);
    }
}
