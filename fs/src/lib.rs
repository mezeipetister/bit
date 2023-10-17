use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    time::{self, SystemTime},
};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Superblock {
    magic: [u8; 7],        // Magic number to check
    block_size: u32,       // Block size in bytes
    blocks_per_group: u32, // max data block per group
    total_groups: u32,     // Total groups count
    total_blocks: u32,     // Total blocks count
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
            total_blocks: 0,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum Block {
    Empty,
    Inode {
        folder: bool,
        created: u64,
        last_accesed: u64,
        size: u64,
        checksum: u32,
        block_count: u32,
        data: Vec<u8>,
        next: (u32, u32),
    },
    Data {
        inode_index: u32,
        position: u32,
        data: Vec<u8>,
        next: (u32, u32),
    },
}
