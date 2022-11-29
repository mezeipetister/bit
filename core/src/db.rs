use crate::{
    context::Context,
    prelude::{BitError, BitResult},
    sync::{Commit, CommitCandidate, Entry, Staging},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{Mutex, MutexGuard},
};
use uuid::Uuid;

const PATH_STAGING: &'static str = "staging";
const PATH_LOCAL: &'static str = "local";
const PATH_REMOTE: &'static str = "remote";
const PATH_INDEX: &'static str = "index";

#[async_trait]
pub trait DataRead {
    type ReadResult;
    async fn read(ctx: &Context) -> BitResult<Self::ReadResult>;
}

#[async_trait]
pub trait DataUpdate {
    type UpdateObj: Serialize;
    async fn write(ctx: &Context, data: Self::UpdateObj) -> BitResult<()>;
}

#[async_trait]
pub trait DataAppend {
    type AppendObj: Serialize;
    async fn write(ctx: &Context, data: Self::AppendObj) -> BitResult<()>;
}

pub struct Database {
    lock: LockedDb,
}

impl Database {
    pub async fn init(ctx: &Context) -> BitResult<Self> {
        Ok(Database {
            lock: LockedDb::default(),
        })
    }
}

#[derive(Default)]
pub struct LockedDb {
    local: LocalData,
    remote: RemoteData,
    staging: StagingData,
    index: IndexData,
}

#[derive(Default)]
pub struct IndexData;

#[derive(Default)]
pub struct LocalData;

#[async_trait]
impl DataRead for LocalData {
    type ReadResult = Vec<CommitCandidate>;
    async fn read(ctx: &Context) -> BitResult<Self::ReadResult> {
        unimplemented!()
    }
}

#[async_trait]
impl DataUpdate for LocalData {
    type UpdateObj = Vec<CommitCandidate>;
    async fn write(ctx: &Context, data: Self::UpdateObj) -> BitResult<()> {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct RemoteData;

#[async_trait]
impl DataRead for RemoteData {
    type ReadResult = Vec<CommitCandidate>;
    async fn read(ctx: &Context) -> BitResult<Self::ReadResult> {
        unimplemented!()
    }
}

#[async_trait]
impl DataAppend for RemoteData {
    type AppendObj = Commit;
    async fn write(ctx: &Context, data: Self::AppendObj) -> BitResult<()> {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct StagingData;

#[async_trait]
impl DataRead for Staging {
    type ReadResult = Vec<Staging>;
    async fn read(ctx: &Context) -> BitResult<Self::ReadResult> {
        unimplemented!()
    }
}

#[async_trait]
impl DataUpdate for Staging {
    type UpdateObj = Vec<Staging>;
    async fn write(ctx: &Context, data: Self::UpdateObj) -> BitResult<()> {
        unimplemented!()
    }
}

impl LockedDb {
    pub fn last_commit_id_remote(&self) -> Uuid {
        unimplemented!()
    }
    pub fn last_commit_id_local(&self) -> Uuid {
        unimplemented!()
    }
    async fn init(&self, ctx: &Context) -> BitResult<()> {
        if ctx.is_bit_project_path() {
            return Err(BitError::new("Already a bit project. Cannot init it again"));
        }
        let data_dir = &ctx.current_dir().join(".bit");
        // Init empty STAGING db file
        let _ = File::create(data_dir.join(".bit").join(PATH_STAGING)).await;
        // Init empty LOCAL db file
        let _ = File::create(data_dir.join(".bit").join(PATH_LOCAL)).await;
        // Init empty REMOTE db file
        let _ = File::create(data_dir.join(".bit").join(PATH_REMOTE)).await;
        // Init empty INDEX db file
        let _ = File::create(data_dir.join(".bit").join(PATH_INDEX)).await;
        Ok(())
    }
    async fn reset(&self, ctx: &Context) -> BitResult<()> {
        // First remove .bit directory
        if !ctx.is_bit_project_path() {
            return Err(BitError::new("Not a BIT project; cannot reset it"));
        }
        // Remove bit data directory
        let _ = tokio::fs::remove_dir_all(ctx.bit_data_path().unwrap())
            .await
            .unwrap();
        // Then init it
        self.init(ctx).await?;
        // Then Ok nothing
        Ok(())
    }
    async fn get_staging(&self, ctx: &Context) -> BitResult<Staging> {
        // Try open staging
        let mut file = OpenOptions::new()
            .read(true)
            .open(ctx.bit_data_path().unwrap().join(PATH_STAGING))
            .await
            .map_err(|_| BitError::new("No staging db file found"))?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        Ok(bincode::deserialize(&contents)?)
    }
    async fn update_staging(&self, ctx: &Context, staging: Staging) -> BitResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(ctx.bit_data_path().unwrap().join(PATH_STAGING))
            .await
            .map_err(|_| BitError::new("No staging db file found"))?;
        file.write_all(&bincode::serialize(&staging).unwrap())
            .await?;
        Ok(())
    }
    async fn add_to_staging(&self, ctx: &Context, entry: Entry) -> BitResult<()> {
        // Get all
        let mut now = self.get_staging(ctx).await?;
        // Add entry
        now.add_entry(entry);
        // Store entries
        self.update_staging(ctx, now).await?;
        Ok(())
    }
    async fn get_local(ctx: &Context) -> BitResult<Vec<CommitCandidate>> {
        // Try open staging
        let mut file = OpenOptions::new()
            .read(true)
            .open(ctx.bit_data_path().unwrap().join(PATH_LOCAL))
            .await
            .map_err(|_| BitError::new("No local db file found"))?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        Ok(bincode::deserialize(&contents)?)
    }
    async fn get_remote(ctx: &Context) -> BitResult<Vec<Commit>> {
        let ctx = ctx.to_owned();
        let res = tokio::task::spawn_blocking(move || {
            let mut res: Vec<Commit> = Vec::new();
            let f = std::fs::File::open(ctx.bit_data_path().unwrap().join(PATH_REMOTE)).unwrap();
            loop {
                match bincode::deserialize_from(&f) {
                    Ok(r) => res.push(r),
                    Err(_) => {
                        break;
                    }
                }
            }
            res
        })
        .await?;
        Ok(res)
    }
}
