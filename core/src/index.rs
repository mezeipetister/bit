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
use chrono::NaiveDate;
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
    pub fn account_exist(&self, id: &str) -> bool {
        self.accounts.iter().find(|a| a.id == id).is_some()
    }
    pub fn account_get(&self, id: &str) -> Result<&Account, CliError> {
        self.accounts
            .iter()
            .find(|a| a.id == id)
            .ok_or(CliError::Error("Account not found".to_string()))
    }
    pub fn account_get_mut(&mut self, id: &str) -> Result<&mut Account, CliError> {
        self.accounts
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or(CliError::Error("Account not found".to_string()))
    }
    pub fn account_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.account_get(&id).is_ok() {
            return Err(CliError::Error("Account id is taken".to_string()));
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
    pub fn account_rename(&mut self, id: &str, name: String) -> Result<(), CliError> {
        let a = self.account_get_mut(id)?;
        a.name = name;
        Ok(())
    }
    pub fn partner_exist(&self, id: &str) -> bool {
        self.partners.iter().find(|a| a.id == id).is_some()
    }
    pub fn partner_get(&self, id: &str) -> Result<&Partner, CliError> {
        self.partners
            .iter()
            .find(|a| a.id == id)
            .ok_or(CliError::Error("Partner not found".to_string()))
    }
    pub fn partner_get_mut(&mut self, id: &str) -> Result<&mut Partner, CliError> {
        self.partners
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or(CliError::Error("Partner not found".to_string()))
    }
    pub fn partner_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.partner_get(&id).is_ok() {
            return Err(CliError::Error("Partner id is taken".to_string()));
        }
        self.partners.push(Partner { id, name });
        Ok(())
    }
    pub fn partner_get_all(&self) -> impl Display {
        (&self.partners).with_title().table().display().unwrap()
    }
    pub fn partner_remove(&mut self, id: &str) -> Result<(), CliError> {
        let _ = self.partner_get(id)?;
        self.partners.retain(|a| a.id != id);
        Ok(())
    }
    pub fn partner_rename(&mut self, id: &str, name: String) -> Result<(), CliError> {
        let a = self.partner_get_mut(id)?;
        a.name = name;
        Ok(())
    }
    pub fn note_get(&self, id: &str) -> Result<&Note, CliError> {
        self.notes
            .iter()
            .find(|n| n.id == Some(id.to_string()))
            .ok_or(CliError::Error("Not found".to_string()))
    }
    pub fn note_get_mut(&mut self, id: &str) -> Result<&mut Note, CliError> {
        self.notes
            .iter_mut()
            .find(|n| n.id == Some(id.to_string()))
            .ok_or(CliError::Error("Not found".to_string()))
    }
    pub fn note_new(&mut self, id: String) -> Result<(), CliError> {
        self.notes.push(Note::new(Some(id)));
        Ok(())
    }
    pub fn note_set(
        &mut self,
        id: &str,
        partner: Option<Partner>,
        description: Option<String>,
        idate: Option<NaiveDate>,
        cdate: Option<NaiveDate>,
        ddate: Option<NaiveDate>,
        net: Option<f32>,
        vat: Option<f32>,
        gross: Option<f32>,
    ) -> Result<(), CliError> {
        let mut note = self.note_get_mut(id)?;
        note.set(partner, description, idate, cdate, ddate, net, vat, gross)
    }
    pub fn note_unset(
        &mut self,
        id: &str,
        partner: bool,
        description: bool,
        idate: bool,
        cdate: bool,
        ddate: bool,
        net: bool,
        vat: bool,
        gross: bool,
    ) -> Result<(), CliError> {
        let mut note = self.note_get_mut(id)?;
        note.unset(partner, description, idate, cdate, ddate, net, vat, gross)
    }
    pub fn note_set_transaction(
        &mut self,
        id: &str,
        debit: Account,
        credit: Account,
        amount: f32,
        comment: Option<String>,
    ) -> Result<(), CliError> {
        let mut note = self.note_get_mut(id)?;
        note.set_transaction(amount, debit, credit, comment)?;
        Ok(())
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        self.save_fs()
    }
}
