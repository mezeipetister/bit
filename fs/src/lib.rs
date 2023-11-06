use anyhow::anyhow;
use bitvec::{order::Lsb0, vec::BitVec};
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use std::any;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Cursor, Seek, SeekFrom};
use std::ops::Deref;
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
const INODE_MAX_REGION: usize = 500;

pub mod util;

#[derive(Debug)]
pub struct FS {
    pub superblock: Superblock,
    pub file: File,
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
        // let mmap = unsafe { MmapMut::map_mut(&file)? };

        let superblock = Superblock::new();

        let mut fs = Self {
            superblock,
            file,
            groups: vec![],
        };

        // Create group
        let group = Group::init();

        // Add to superblock
        fs.add_group(group)?;

        // Save superblock
        fs.save_superblock()?;

        Ok(fs)
    }

    /// Open FS from a given path
    pub fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        // Open image path as read & write
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path.as_ref())?;

        let mut r = BufReader::new(&mut file);

        // Deserialize superblock from cursor
        let superblock: Superblock = Superblock::deserialize_from(&mut r)?;

        let mut groups = vec![];

        // Deserialize groups based on superblock group count
        for group_index in 0..superblock.block_count {
            let group = Group::deserialize_from(&mut r, group_index)?;
            groups.push(group);
        }

        let fs = Self {
            superblock,
            groups,
            file,
        };

        // Return FS
        Ok(fs)
    }

    // #[inline]
    // fn find_inode_by_path<P>(&self, p: P) -> anyhow::Result<Inode>
    // where
    //     P: AsRef<Path>,
    // {
    //     let mut r = Cursor::new(self.mmap());

    //     // Read root inode
    //     let root_inode = Inode::deserialize_from(r, ROOT_INODE)?;

    //     // Check if its a folder
    //     assert!(root_inode.folder);

    //     let mut raw_data = vec![];
    //     let mut w = BufWriter::new(&mut raw_data);

    //     // Read raw data
    //     let checksum = self.read_inode_data(&root_inode, &mut w)?;

    //     // Check checksum
    //     assert_eq!(checksum, root_inode.data_checksum);

    //     // Deserialize folders
    //     let dir: DirectoryIndex = bincode::deserialize(&raw_data)?;
    // }

    #[inline]
    fn save_superblock(&mut self) -> anyhow::Result<()> {
        let mut w = BufWriter::new(&self.file);
        self.superblock.checksum();
        let mut data = bincode::serialize(&self.superblock)?;
        w.seek(SeekFrom::Start(0))?;
        w.write_all(&mut data)?;
        Ok(())
    }

    #[inline]
    pub fn get_inode(&self, inode_block_index: u32) -> anyhow::Result<Inode> {
        let mut r = BufReader::new(&self.file);

        r.seek(SeekFrom::Start(
            block_seek_position(inode_block_index) as u64
        ))?;

        // Deserialize by bincode
        let inode: Inode = Inode::deserialize_from(r)?;

        // Return inode
        Ok(inode)
    }

    #[inline]
    pub fn save_inode(&mut self, inode: &mut Inode) -> anyhow::Result<()> {
        let mut w = BufWriter::new(&self.file);

        w.seek(SeekFrom::Start(
            block_seek_position(inode.block_index) as u64
        ));
        inode.set_last_modified();
        inode.serialize_into(w)?;
        Ok(())
    }

    #[inline]
    fn save_group(&mut self, group: &Group, group_index: u32) -> anyhow::Result<()> {
        let mut w = BufWriter::new(&self.file);

        w.seek(SeekFrom::Start(Group::seek_position(group_index) as u64));
        group.serialize_into(w)?;
        Ok(())
    }

    #[inline]
    pub fn read_inode_data<W>(&self, inode: &Inode, mut w: &mut W) -> anyhow::Result<u32>
    where
        W: Write,
    {
        let mut checksum = Checksum::new();
        let mut r = BufReader::new(&self.file);

        match &inode.data {
            Data::Raw(data) => {
                checksum.update(&data);
                w.write_all(&data)?;
            }
            Data::DirectPointers(pointers) => {
                // Counting data left to read
                let mut data_left = inode.size;

                for (block_index, range) in pointers {
                    // Seek start position
                    r.seek(SeekFrom::Start(block_seek_position(*block_index) as u64))?;

                    // Create buffer for range
                    let len = match data_left > BLOCK_SIZE as u64 {
                        true => (range * BLOCK_SIZE) as usize,
                        false => data_left as usize,
                    };
                    let mut buf = Vec::with_capacity(len);
                    unsafe { buf.set_len(len) };

                    // Read range bytes
                    r.get_mut().read_exact(&mut buf)?;

                    // Update checksum
                    checksum.update(&buf);

                    // Write buffer to writer
                    std::io::copy(&mut Cursor::new(buf), &mut w);

                    // Decrease data_left
                    data_left -= len as u64;
                }
            }
        }

        Ok(checksum.finalize())
    }

    #[inline]
    pub fn write_inode_data<R>(
        &mut self,
        inode: &mut Inode,
        mut data: &mut R,
        data_len: u64,
    ) -> anyhow::Result<()>
    where
        R: BufRead,
    {
        // If data length fits inside inode
        if data_len as usize <= INODE_CAPACITY {
            // Set data inside inode
            inode.set_raw_data(&mut data, data_len)?;

            // Save inode
            self.save_inode(inode)?;

            // Return ok
            return Ok(());
        }

        // If data does not fit inside Inode as raw data

        // Set inode data size
        inode.size = data_len;
        // And save it
        self.save_inode(inode)?;

        // Define empty ranges
        let mut ranges: Vec<(u32, u32)> = vec![];

        // Define block_to_allocate
        let mut blocks_to_allocate = |data_size| {
            data_size / BLOCK_SIZE as u64 + u64::from(data_size % BLOCK_SIZE as u64 != 0)
        };

        // Determine how many block we need
        let mut block_to_allocate = blocks_to_allocate(data_len);

        let mut groups = self.groups.clone();

        for (group_index, group) in groups.iter_mut().enumerate() {
            // Check if we need any blocks?
            if block_to_allocate > 0 {
                // Allocate regions from group
                let (mut range, left) = group.allocate_region(
                    group_index as u32,
                    block_to_allocate as usize,
                    INODE_MAX_REGION,
                );

                // Save group
                self.save_group(&group, group_index as u32)?;

                ranges.append(&mut range);

                // Decrease block wanted
                block_to_allocate = left as u64;
            }
        }

        // Write data into ranges
        let mut data_left = data_len;

        let mut w = BufWriter::new(&self.file);

        for (block_index, range) in ranges {
            // Determine chunk size
            let chunk_size = match data_left > BLOCK_SIZE as u64 {
                true => (range * BLOCK_SIZE) as usize,
                false => data_left as usize,
            };

            // Create buffer chunk
            let mut buf = Vec::with_capacity(chunk_size);
            unsafe { buf.set_len(chunk_size) };

            // Read data into chunk buffer
            data.read_exact(&mut buf)?;

            // Seek position
            w.seek(SeekFrom::Start(block_seek_position(block_index) as u64))?;

            // Write chunk buffer to disk
            w.write_all(&mut buf)?;

            // Decrease data left
            data_left -= chunk_size as u64;
        }

        // Check all data has written
        assert!(data_left == 0);

        // Flush disk
        w.flush()?;

        Ok(())
    }

    #[inline]
    fn truncate(&mut self) -> anyhow::Result<()> {
        // Superblock + GroupCount * (Group bitmap + group data inodes)
        let size =
            BLOCK_SIZE + (self.groups.len() as u32) * (BLOCK_SIZE + BLOCKS_PER_GROUP * BLOCK_SIZE);
        // Set file size
        self.file.set_len(size as u64)?;
        // Return ok
        Ok(())
    }

    #[inline]
    fn add_group(&mut self, group: Group) -> anyhow::Result<()> {
        // Save group to disk
        self.save_group(&group, self.groups.len() as u32 + 1)?;
        // Insert new group to FS groups
        self.groups.push(group);
        // Truncate itself
        self.truncate()?;
        // Return ok
        Ok(())
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

#[derive(Debug, Default, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Inode {
    pub block_index: u32,
    pub created: u64,
    pub last_modified: u64,
    pub size: u64,
    pub data_checksum: u32,
    pub data: Data,
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

impl Inode {
    pub fn new(block_index: u32) -> Self {
        Self {
            block_index,
            created: now(),
            last_modified: now(),
            size: 0,
            data_checksum: calculate_checksum(&()),
            data: Data::Raw(vec![]),
        }
    }

    #[inline]
    pub fn serialize_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: Write + Seek,
    {
        // Serialize inode bytes array
        let serialized = bincode::serialize(&self)?;

        // Check if serialized inode size is correct
        assert!(serialized.len() as u32 <= BLOCK_SIZE);

        // Write serialized inode
        w.write_all(&serialized)?;

        // Flush buffer
        w.flush()?;

        Ok(())
    }

    #[inline]
    pub fn deserialize_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: Read + Seek,
    {
        let inode: Inode = bincode::deserialize_from(&mut r)?;
        Ok(inode)
    }

    #[inline]
    fn set_last_modified(&mut self) {
        self.last_modified = now();
    }

    #[inline]
    fn set_raw_data<R>(&mut self, data: &mut R, data_size: u64) -> anyhow::Result<()>
    where
        R: BufRead,
    {
        let mut buffer = vec![];
        let data_len = data.read(&mut buffer)?;

        if data_len != data_size as usize {
            return Err(anyhow!("Data read and given data size are not the same"));
        }

        if data_len > INODE_CAPACITY as usize {
            return Err(anyhow!(
                "Data is too big to be raw data. Does not fit inside inode"
            ));
        }

        self.size = data_size;
        self.data = Data::Raw(buffer);
        Ok(())
    }

    #[inline]
    fn data<R, W>(&self) -> &Data {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DirectoryIndex {
    pub directories: BTreeMap<OsString, u32>,
    checksum: u32,
}

impl DirectoryIndex {
    fn init() -> anyhow::Result<Self> {
        let mut dir = DirectoryIndex {
            directories: BTreeMap::new(),
            checksum: 0,
        };
        dir.checksum();
        Ok(dir)
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Directory {
    pub directories: BTreeMap<OsString, u32>,
    pub files: BTreeMap<String, u32>,
    checksum: u32,
}

impl Directory {
    fn init() -> anyhow::Result<Self> {
        let mut dir = Directory {
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
            checksum: 0,
        };
        dir.checksum();
        Ok(dir)
    }

    pub fn get_file(&self, file_name: &str) -> Option<u32> {
        self.files.get(file_name).map(|x| *x)
    }

    pub fn add_file(&mut self, file_name: &str, inode_block_index: u32) -> anyhow::Result<()> {
        match self.get_file(file_name) {
            Some(_) => Err(anyhow!("File already exist")),
            None => {
                self.files.insert(file_name.into(), inode_block_index);
                Ok(())
            }
        }
    }

    pub fn get_folder<P>(&self, path: P) -> Option<u32>
    where
        P: AsRef<Path>,
    {
        self.directories
            .get(&path.as_ref().as_os_str().to_os_string())
            .map(|x| *x)
    }

    pub fn add_folder(
        &mut self,
        folder_name: &OsString,
        inode_block_index: u32,
    ) -> anyhow::Result<()> {
        match self.get_folder(folder_name) {
            Some(_) => Err(anyhow!("Folder already exist")),
            None => {
                self.directories
                    .insert(folder_name.into(), inode_block_index);
                Ok(())
            }
        }
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
