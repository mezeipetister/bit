use crate::{
    account::Account,
    blob::Blob,
    context::{Context, CtxError},
    fs::{
        binary_init, binary_init_empty, binary_read, binary_update, cwd, is_project_cwd, FsError,
    },
    ledger::Ledger,
    note::Note,
    partner::Partner,
    prelude::{path_helper, CliDisplay, CliError},
};
use cli_table::{Table, WithTitle};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum IndexError {
    AlreadyInited,
    NotRepo,
    Fs(FsError),
    CtxError(CtxError),
}

impl Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::AlreadyInited => write!(f, "Path already an inited repo"),
            IndexError::NotRepo => write!(f, "Path is not a repository."),
            IndexError::Fs(fs) => write!(f, "{}", fs.to_string()),
            IndexError::CtxError(ctx) => write!(f, "{}", ctx.to_string()),
        }
    }
}

impl From<IndexError> for CliError {
    fn from(f: IndexError) -> Self {
        Self::Error(f.to_string())
    }
}

#[derive(Debug)]
pub struct Db {
    ctx: Context,
    inner: DbInner,
}

impl Deref for Db {
    type Target = DbInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Db {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Db {
    pub fn init() -> Result<Self, IndexError> {
        let inner = DbInner::init()?;
        let ctx = Context::new()?;
        Ok(Self { ctx, inner })
    }
    pub fn load() -> Result<Self, IndexError> {
        let ctx = Context::new()?;
        let inner = DbInner::load(&ctx)?;
        Ok(Self { ctx, inner })
    }
    fn save_fs(&self) {
        binary_update(path_helper::index(&self.ctx), &self.inner)
            .expect("Error writing bit db to fs");
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbInner {
    accounts: Vec<Account>,
    notes: Vec<Note>,
    ledger: Ledger,
    partners: Vec<Partner>,
    blobs: Vec<Blob>,
}

impl CliDisplay for DbInner {
    fn display(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(f, "{:?}", self)
    }
}

impl DbInner {
    fn init() -> Result<Self, IndexError> {
        if is_project_cwd() {
            return Err(IndexError::AlreadyInited);
        }
        let ctx = Context::new_cwd();
        // Init index file
        binary_init(path_helper::index(&ctx), DbInner::default())?;
        // Init blob file
        binary_init_empty(path_helper::blob_database(&ctx))?;
        // Load self after init
        Self::load(&ctx)
    }
    fn load(ctx: &Context) -> Result<Self, IndexError> {
        // Try to read database
        let res = binary_read(path_helper::index(ctx))?;
        Ok(res)
    }
    pub fn account_get(&self, id: &str) -> Result<&Account, CliError> {
        self.accounts
            .iter()
            .find(|a| a.id == id)
            .ok_or(CliError::Error("Not found".to_string()))
    }
    pub fn account_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.account_get(&id).is_ok() {
            return Err(CliError::Error("Account is taken".to_string()));
        }
        self.accounts.push(Account { id, name });
        Ok(())
    }
    pub fn account_get_all(&self) -> impl Display {
        (&self.accounts).with_title().table().display().unwrap()
    }
    pub fn account_remove(&mut self, id: &str) -> Result<(), CliError> {
        let _ = self.account_get(id)?;
        self.accounts.retain(|a| a.id != id);
        Ok(())
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        self.save_fs()
    }
}
