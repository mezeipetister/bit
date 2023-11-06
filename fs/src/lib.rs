use anyhow::anyhow;
use bitvec::{order::Lsb0, vec::BitVec};
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Cursor, Seek, SeekFrom};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    io::{Read, Write},
    path::Path,
};

use util::*;

// const M: u32 = 0xb1a9;
const MAGIC: [u8; 7] = *b"*bitfs*";
const FS_VERSION: u32 = 1;
const ROOT_INODE: u32 = 1;
const BLOCK_SIZE: u32 = 4096;
const BLOCKS_PER_GROUP: u32 = BLOCK_SIZE * 8;
const INODE_CAPACITY: usize = 4047;
const CHUNK_CAPACITY: usize = 4076;

pub mod util;

#[derive(Debug)]
pub struct FS {
    pub superblock: Superblock,
    pub mmap: MmapMut,
    pub groups: Vec<Group>,
}

impl FS {
    /// Init FS to a given path
    pub fn init<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        // Create path if it has not exist (yet)
        // Fails if path does exist
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path.as_ref())?;

        // Create mmap from file
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        let superblock = Superblock::new();

        let groups = vec![];

        let mut res = Self {
            superblock,
            mmap,
            groups,
        };

        Ok(res)
    }

    /// Open FS from a given path
    pub fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        // Open image path as read & write
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path.as_ref())?;

        // Create mmap from file
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        // Set cursor around mmap
        let mut cursor = Cursor::new(&mmap);

        // Deserialize superblock from cursor
        let superblock: Superblock = Superblock::deserialize_from(&mut cursor)?;

        let mut groups = vec![];

        // Deserialize groups based on superblock group count
        for group_index in 0..superblock.block_count {
            let group = Group::deserialize_from(&mut cursor, group_index)?;
            groups.push(group);
        }

        let fs = Self {
            superblock,
            groups,
            mmap,
        };

        // Return FS
        Ok(fs)
    }

    #[inline]
    fn groups(&self) -> &[Group] {
        &self.groups
    }

    #[inline]
    fn groups_mut(&mut self) -> &mut [Group] {
        &mut self.groups
    }

    #[inline]
    fn superblock(&self) -> &Superblock {
        &self.superblock
    }

    #[inline]
    fn superblock_mut(&mut self) -> &mut Superblock {
        &mut self.superblock
    }

    #[inline]
    fn mmap(&self) -> &MmapMut {
        &self.mmap
    }

    #[inline]
    fn mmap_mut(&mut self) -> &mut MmapMut {
        &mut self.mmap
    }

    #[inline]
    fn save_inode(&mut self, mut inode: Inode, inode_block_index: u32) -> anyhow::Result<()> {
        let offset = block_seek_position(inode_block_index) as u64;
        let buf = self.mmap_mut().as_mut();
        let mut cursor = Cursor::new(buf);
        cursor.seek(SeekFrom::Start(offset))?;

        Ok(inode.serialize_into(&mut cursor)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Superblock {
    magic: [u8; 7],   // Magic number to check
    fs_version: u32,  // FS Version
    block_size: u32,  // Block size in bytes
    group_count: u32, // Total groups count
    block_count: u32, // Total blocks count
    free_blocks: u32, // Available blocks
    file_count: u32,  // File count in fs
    created: u64,     // FS creation time
    modified: u64,    // FS last modification time
    checksum: u32,    // Superblock checksum
}

impl Superblock {
    fn new() -> Self {
        Self {
            magic: MAGIC,
            fs_version: FS_VERSION,
            block_size: BLOCK_SIZE,
            group_count: 0,
            block_count: 1,
            free_blocks: 0,
            file_count: 0,
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

    #[inline]
    pub fn serialize_into<W>(&mut self, w: W) -> anyhow::Result<()>
    where
        W: Write,
    {
        self.checksum();
        bincode::serialize_into(w, self).map_err(|e| e.into())
    }

    #[inline]
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

    #[inline]
    fn checksum(&mut self) {
        self.checksum = 0;
        self.checksum = calculate_checksum(&self);
    }

    #[inline]
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
}

impl Group {
    fn new(block_bitmap: BitVec<u8, Lsb0>) -> Self {
        Self { block_bitmap }
    }

    fn init() -> Self {
        let mut block_bitmap = BitVec::<u8, Lsb0>::with_capacity(BLOCK_SIZE as usize * 8);
        block_bitmap.resize(BLOCK_SIZE as usize * 8, false);
        Self { block_bitmap }
    }

    #[inline]
    fn seek_position(group_index: u32) -> u32 {
        // Superblock BLOCK_SIZE (4kib)
        // + Group ID * (BLOCK_SIZE + BLOCKS_PER_GROUP * BLOCK_SIZE)
        BLOCK_SIZE + group_index * (BLOCK_SIZE + BLOCKS_PER_GROUP * BLOCK_SIZE)
    }

    #[inline]
    fn create_public_address(group_index: u32, bitmap_index: u32) -> u32 {
        // Maybe +1?
        Self::seek_position(group_index) / BLOCK_SIZE + bitmap_index
    }

    // Returns (group_index, inner_block_index)
    #[inline]
    fn translate_public_address(block_index: u32) -> (u32, u32) {
        let inodes_per_group = BLOCKS_PER_GROUP;
        let inode_bg = (block_index as u32 - 1) / inodes_per_group;
        let bitmap_index = (block_index as u32 - 1) & (inodes_per_group - 1);
        (inode_bg, bitmap_index)
    }

    #[inline]
    pub fn serialize_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: Write + Seek,
    {
        w.write_all(self.block_bitmap.as_raw_slice())?;

        Ok(())
    }

    #[inline]
    pub fn deserialize_from<R>(mut r: R, group_index: u32) -> anyhow::Result<Group>
    where
        R: Read + Seek,
    {
        let mut buf = Vec::with_capacity(BLOCK_SIZE as usize);
        unsafe {
            buf.set_len(BLOCK_SIZE as usize);
        }

        let offset = Self::seek_position(group_index);
        r.seek(SeekFrom::Start(offset as u64))?;
        r.read_exact(&mut buf)?;
        let data_bitmap = BitVec::<u8, Lsb0>::from_slice(&buf);

        Ok(Group::new(data_bitmap))
    }

    // #[inline]
    // pub fn has_data_block(&self, i: usize) -> bool {
    //     self.block_bitmap.get(i - 1).as_deref().unwrap_or(&false) == &true
    // }

    #[inline]
    pub fn free_data_blocks(&self) -> usize {
        self.block_bitmap.count_zeros()
    }

    #[inline]
    pub fn total_data_blocks(&self) -> usize {
        self.block_bitmap.len()
    }

    #[inline]
    fn release_one(&mut self, bitmap_index: u32) {
        self.block_bitmap.set(bitmap_index as usize, false);
    }

    #[inline]
    pub fn release_data_region(&mut self, bitmap_index: u32, length: u32) {
        for i in bitmap_index..(bitmap_index + length) {
            self.block_bitmap.set(i as usize, false);
        }
    }

    /// Allocate one block
    #[inline]
    fn allocate_one(&mut self, group_index: u32) -> Option<u32> {
        // If we have at least one free block index
        if let Some(bitmap_index) = self.block_bitmap.iter_zeros().next() {
            // Set it to be taken
            self.block_bitmap.set(bitmap_index, true);
            // Return index as public address
            return Some(Self::create_public_address(
                group_index,
                bitmap_index as u32,
            ));
        }
        None
    }

    /// Allocate data region
    #[inline]
    fn allocate_region(
        &mut self,
        // to translate internal ID into public address
        group_index: u32,
        // Blocks to allocate
        mut blocks_to_allocate: usize,
        // Maximum number of region to allocate
        max_regions: usize,
    ) -> (Vec<(u32, u32)>, usize) {
        let mut regions = Vec::new();
        let mut region: Option<(u32, u32)> = None;

        let mut iter = self.block_bitmap.iter_mut().enumerate().peekable();

        while let Some((bitmap_index, mut i)) = iter.next() {
            // Break loop if we dont need more blocks
            // to allocate
            if blocks_to_allocate == 0 {
                // Add opened region to regions if we have one opened
                if let Some(r) = region.take() {
                    regions.push(r);
                }
                break;
            }

            // If current block index is free
            if !*i {
                // If we have opened region
                if let Some((block_index, region_length)) = region.as_mut() {
                    // Then increment region_length
                    *region_length += 1;
                } else {
                    // Else we need to create a new opened region
                    region = Some((
                        Self::create_public_address(group_index, bitmap_index as u32),
                        1,
                    ));
                }

                // Decrease blocks number to allocate by one
                // As we allocate on in this if block
                blocks_to_allocate -= 1;

                // Set block index as taken
                i.set(true);

                // If i is taken
            } else {
                // Check if we have opened region
                // and close it
                if let Some(r) = region.take() {
                    regions.push(r);

                    // Break loop if we reached the maximum region number
                    // we dont have room to allocate more regions
                    if regions.len() == max_regions {
                        break;
                    }
                }
            }

            // If last item, then clean up
            if let None = iter.peek() {
                // If we have opened region
                // then close it
                if let Some(r) = region.take() {
                    regions.push(r);
                }
            }
        }

        // allocated regions
        //  |                  remaining blocks to allocate
        //  |                     |
        (regions, blocks_to_allocate)
    }

    // #[inline]
    // fn next_free_data_region(&self, size: u32) -> Option<(usize, usize)> {
    //     self.block_bitmap
    //         .windows(size as usize)
    //         .position(|p| p.not_any())
    //         .map(|p| (p + 1, p + size as usize + 1))
    // }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Inode {
    pub folder: bool,
    pub created: u64,
    pub last_accesed: u64,
    pub size: u64,
    pub checksum: u32,
    pub data: Data,
    pub next: (u32, u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    Raw(Vec<u8>),
    DirectPointers(Vec<(u32, u32)>),
}

impl Default for Data {
    fn default() -> Self {
        Self::Raw(vec![])
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

    #[test]
    fn test_block_address() {
        let group_index = 0;
        let bitmap_index = 3;
        let block_index = Group::create_public_address(group_index, bitmap_index);
        let (_group_index, _bitmap_index) = Group::translate_public_address(block_index);
        assert_eq!(group_index, _group_index);
        assert_eq!(bitmap_index, _bitmap_index);
    }

    #[test]
    fn test_block_allocation() {
        let mut group = Group::init();

        for i in [2, 20, 1500, 2000, 2002] {
            group.block_bitmap.set(i, true);
        }

        let res = group.allocate_region(0, 1000, 500);
        println!("{:?}", res);
        assert_eq!(res.0.len(), 3);
        assert_eq!(res.1, 0);

        // Test to release allocated regions
        for (address, length) in res.0 {
            let (block_index, bitmap_index) = Group::translate_public_address(address);
            println!("{}", bitmap_index);
            group.release_data_region(bitmap_index, length);
        }

        for i in [2, 20, 1500, 2000, 2002] {
            group.block_bitmap.set(i, false);
        }

        assert_eq!(group.free_data_blocks() as u32, BLOCKS_PER_GROUP);

        let res = group.allocate_region(0, 40000, 200);

        // Test to release allocated regions
        for (address, length) in res.0 {
            let (block_index, bitmap_index) = Group::translate_public_address(address);
            group.release_data_region(bitmap_index, length);
        }

        assert_eq!(group.free_data_blocks() as u32, BLOCKS_PER_GROUP);
    }
}
