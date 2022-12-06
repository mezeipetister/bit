use crate::{
    context::Context,
    core::{Commit, CommitCandidate, Entry, Staging},
    fs::*,
    index::Index,
    prelude::{BitError, BitResult},
};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    sync::{Mutex, MutexGuard},
};
use uuid::Uuid;

const PATH_STAGING: &'static str = "staging";
const PATH_LOCAL: &'static str = "local";
const PATH_REMOTE: &'static str = "remote";
const PATH_INDEX: &'static str = "index";

pub trait Db {
    type DataType: for<'de> Deserialize<'de> + Serialize + Send + Sync;
    const DB_PATH: &'static str;
}

pub struct Database {
    inner_lock: Mutex<LockedDb>,
}

impl Database {
    pub async fn load() -> Self {
        Database {
            inner_lock: Mutex::new(LockedDb::default()),
        }
    }
    pub async fn lock(&self) -> MutexGuard<LockedDb> {
        self.inner_lock.lock().await
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

impl Db for IndexData {
    type DataType = Index;
    const DB_PATH: &'static str = PATH_INDEX;
}

impl DataRead for IndexData {}
impl DataUpdate for IndexData {}

#[derive(Default)]
pub struct LocalData;

impl Db for LocalData {
    type DataType = Vec<CommitCandidate>;
    const DB_PATH: &'static str = PATH_LOCAL;
}

impl DataRead for LocalData {}
impl DataUpdate for LocalData {}

#[derive(Default)]
pub struct RemoteData;

impl Db for RemoteData {
    type DataType = Vec<Commit>;
    const DB_PATH: &'static str = PATH_REMOTE;
}

impl DataIter for RemoteData {
    type IterOutputType = Commit;
}
impl DataUpdate for RemoteData {}
impl DataAppend for RemoteData {
    type AppendDataType = Commit;
}

#[derive(Default)]
pub struct StagingData;

impl Db for StagingData {
    type DataType = Staging;
    const DB_PATH: &'static str = PATH_STAGING;
}

impl DataRead for StagingData {}
impl DataUpdate for StagingData {}

impl LockedDb {
    pub async fn last_commit_id_remote(&self, ctx: &Context) -> BitResult<Option<Uuid>> {
        let res = RemoteData::read(ctx).await?.last().map(|l| l.data().id());
        Ok(res)
    }
    pub async fn last_commit_id_local(&self, ctx: &Context) -> BitResult<Option<Uuid>> {
        let res = LocalData::read(ctx).await?.last().map(|l| l.id());
        Ok(res)
    }
    pub async fn last_commit_id(&self, ctx: &Context) -> BitResult<Uuid> {
        match self.last_commit_id_local(ctx).await? {
            // Send back last local if we have one
            Some(lcil) => Ok(lcil),
            None => match self.last_commit_id_remote(ctx).await? {
                // Or send back last remote if we have one
                Some(lcir) => Ok(lcir),
                // Or create a phantom UUID and fix it later
                None => Ok(Uuid::new_v4()),
            },
        }
    }
    pub async fn init(&self) -> BitResult<()> {
        // Check whether already a bit project
        match Context::new(crate::context::Mode::Setup) {
            Ok(_) => return Err(BitError::new("Already a bit project. Cannot init it again")),
            Err(_) => {
                let cwd = std::env::current_dir().unwrap();
                tokio::fs::create_dir_all(cwd.join(".bit")).await?;
                // Init context again
                let ctx = Context::new(crate::context::Mode::Setup).unwrap();
                // Init empty STAGING db file
                let _ = File::create(ctx.bit_data_path().join(PATH_STAGING)).await;
                self.reset_staging(&ctx).await?;
                // Init empty LOCAL db file
                let _ = File::create(ctx.bit_data_path().join(PATH_LOCAL)).await;
                self.reset_local(&ctx).await?;
                // Init empty REMOTE db file
                let _ = File::create(ctx.bit_data_path().join(PATH_REMOTE)).await;
                self.reset_remote(&ctx).await?;
                // Init empty INDEX db file
                let _ = File::create(ctx.bit_data_path().join(PATH_INDEX)).await;
                self.reset_index(&ctx).await?;
                Ok(())
            }
        }
    }
    // async fn reset(&self, ctx: &Context) -> BitResult<()> {
    //     // First remove .bit directory
    //     if !ctx.is_bit_project_path() {
    //         return Err(BitError::new("Not a BIT project; cannot reset it"));
    //     }
    //     // Remove bit data directory
    //     let _ = tokio::fs::remove_dir_all(ctx.bit_data_path().unwrap())
    //         .await
    //         .unwrap();
    //     // Then init it
    //     self.init(ctx).await?;
    //     // Then Ok nothing
    //     Ok(())
    // }
    async fn get_staging(&self, ctx: &Context) -> BitResult<Staging> {
        StagingData::read(ctx).await
    }
    async fn add_to_staging(&self, ctx: &Context, entry: Entry) -> BitResult<()> {
        // Get all
        let mut now = self.get_staging(ctx).await?;
        // Add entry
        now.add_entry(entry);
        // Store entries
        StagingData::update(ctx, now).await?;
        Ok(())
    }
    async fn reset_staging(&self, ctx: &Context) -> BitResult<()> {
        let s = Staging::default();
        StagingData::update(ctx, s).await?;
        Ok(())
    }
    async fn get_local(&self, ctx: &Context) -> BitResult<Vec<CommitCandidate>> {
        LocalData::read(ctx).await
    }
    async fn reset_local(&self, ctx: &Context) -> BitResult<()> {
        LocalData::update(ctx, vec![]).await
    }
    async fn get_remote(&self, ctx: &Context) -> BitResult<Vec<Commit>> {
        RemoteData::read(ctx).await
    }
    async fn get_index(&self, ctx: &Context) -> BitResult<Index> {
        IndexData::read(ctx).await
    }
    async fn reset_remote(&self, ctx: &Context) -> BitResult<()> {
        RemoteData::update(ctx, vec![]).await
    }
    async fn reset_index(&self, ctx: &Context) -> BitResult<()> {
        let mut data = Index::default();
        data.commit_count = 4;
        IndexData::update(ctx, data).await
    }
    async fn re_index(&self, ctx: &Context) -> BitResult<()> {
        IndexData::update(
            ctx,
            Index::default().build(
                self.get_remote(ctx).await?,
                self.get_local(ctx).await?,
                self.get_staging(ctx).await?,
            )?,
        )
        .await?;
        Ok(())
    }
    async fn commit(&self, ctx: &Context, message: String) -> BitResult<()> {
        let mut local_commits = self.get_local(ctx).await?;
        let staging = self.get_staging(ctx).await?;
        local_commits.push(CommitCandidate::from_staging(
            ctx,
            &staging,
            message,
            self.last_commit_id(ctx).await?,
        ));
        LocalData::update(ctx, local_commits).await?;
        self.reset_staging(ctx).await?;
        self.re_index(ctx).await?;
        Ok(())
    }
    // Pull
    // Push
}

#[cfg(test)]
mod tests {
    use crate::context::Mode;

    use super::*;

    #[tokio::test]
    async fn test_db_init() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load().await.lock().await.init().await.unwrap();
    }

    #[tokio::test]
    async fn test_db_add_entry() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .add_to_staging(&ctx, Entry::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_db_commit() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .commit(&ctx, "demo".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_staging_reset() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .reset_staging(&ctx)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_reset_index() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .reset_index(&ctx)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_re_index() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .re_index(&ctx)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_index() {
        let ctx = Context::new(Mode::Local).unwrap();
        let res = Database::load()
            .await
            .lock()
            .await
            .get_index(&ctx)
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_remote() {
        let ctx = Context::new(Mode::Local).unwrap();
        Database::load()
            .await
            .lock()
            .await
            .get_remote(&ctx)
            .await
            .unwrap();
    }
}
