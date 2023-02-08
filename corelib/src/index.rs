use crate::{
    account::Account,
    actions::BitAction,
    blob::Blob,
    context::{Context, CtxError},
    fs::{binary_init, binary_init_empty, binary_read, binary_update, is_project_cwd, FsError},
    ledger::{Ledger, MonthlySummary},
    note::Note,
    partner::Partner,
    prelude::{path_helper, CliDisplay, CliError},
};
use chrono::NaiveDate;
use cli_table::{Table, WithTitle};
use repository::sync::{DocRefVec, Mode, Repository};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug)]
pub enum IndexError {
    AlreadyInited,
    NotRepo,
    Fs(FsError),
    CtxError(CtxError),
    SyncError(String),
}

impl Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::AlreadyInited => write!(f, "Path already an inited repo"),
            IndexError::NotRepo => write!(f, "Path is not a repository."),
            IndexError::Fs(fs) => write!(f, "{}", fs.to_string()),
            IndexError::CtxError(ctx) => write!(f, "{}", ctx.to_string()),
            IndexError::SyncError(e) => write!(f, "{e}"),
        }
    }
}

impl From<IndexError> for CliError {
    fn from(f: IndexError) -> Self {
        Self::Error(f.to_string())
    }
}

impl From<String> for IndexError {
    fn from(value: String) -> Self {
        Self::SyncError(value)
    }
}

#[derive(Debug)]
pub struct IndexDb {
    ctx: Context,
    inner: IndexInner,
    repository: Repository,
}

impl Deref for IndexDb {
    type Target = IndexInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for IndexDb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl IndexDb {
    pub fn init(mode: Mode) -> Result<Self, IndexError> {
        let inner = IndexInner::init()?;
        let ctx = Context::new()?;
        let repository = Repository::init(
            repository::sync::Context::init(ctx.bitdir_path().join("sync"), "demo".to_string()), // TODO! Implement UID
            mode,
        )?;
        let mut index = Self {
            ctx,
            inner,
            repository,
        };
        Ok(index)
    }
    pub fn load() -> Result<Self, IndexError> {
        let ctx = Context::new()?;
        let inner = IndexInner::load(&ctx)?;
        let repository = Repository::load(repository::sync::Context::init(
            ctx.bitdir_path().join("sync"),
            "demo".to_string(), // TODO! Implement UID
        ))?;
        Ok(Self {
            ctx,
            inner,
            repository,
        })
    }
    pub fn pull(&mut self) -> Result<(), IndexError> {
        let _ = self.repository.proceed_pull(&mut self.inner)?;
        Ok(())
    }
    pub fn push(&mut self) -> Result<(), IndexError> {
        let _ = self.repository.proceed_push(&mut self.inner)?;
        Ok(())
    }
    fn save_fs(&self) {
        binary_update(path_helper::index(&self.ctx), &self.inner)
            .expect("Error writing bit db to fs");
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IndexInner {
    accounts: DocRefVec<Account, BitAction>,
    notes: DocRefVec<Note, BitAction>,
    ledger: Ledger,
    partners: DocRefVec<Partner, BitAction>,
}

impl repository::sync::IndexExt for IndexInner {
    type ActionType = BitAction;
    fn reset_docrefs(&mut self) -> Result<(), String> {
        self.accounts.reset();
        self.notes.reset();
        self.partners.reset();
        Ok(())
    }

    fn add_aob(
        &mut self,
        aob: repository::sync::ActionObject<Self::ActionType>,
    ) -> Result<(), String> {
        let res = match aob.storage_id.as_str() {
            "accounts" => self.accounts.add_aob(&aob),
            "notes" => self.notes.add_aob(&aob),
            "partners" => self.partners.add_aob(&aob),
            _ => panic!("No storage found by id"),
        };
        Ok(())
    }
}

impl CliDisplay for IndexInner {
    fn display(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(f, "{:?}", self)
    }
}

impl IndexInner {
    fn init() -> Result<Self, IndexError> {
        if is_project_cwd() {
            return Err(IndexError::AlreadyInited);
        }
        let ctx = Context::new_cwd();
        // Init index file
        binary_init(path_helper::index(&ctx), IndexInner::default())?;
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
        self.accounts.iter().find(|a| a.id() == id).is_some()
    }
    pub fn account_sort(&mut self) {
        self.accounts.sort_by(|a, b| a.id().cmp(&b.id()));
    }
    pub fn account_get(&self, id: &str) -> Result<&Account, CliError> {
        self.accounts
            .iter()
            .find(|a| a.deref().id() == id)
            .map(|i| i.deref())
            .ok_or(CliError::Error("Account not found".to_string()))
    }
    pub fn account_get_mut(&mut self, id: &str) -> Result<&mut Account, CliError> {
        self.accounts
            .iter_mut()
            .find(|a| a.id() == id)
            .map(|i| i.deref_mut())
            .ok_or(CliError::Error("Account not found".to_string()))
    }
    pub fn account_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.account_get(&id).is_ok() {
            return Err(CliError::Error("Account id is taken".to_string()));
        }
        // todo! Implement
        // self.accounts.push(Account { id, name });
        self.account_sort();
        Ok(())
    }
    pub fn account_get_all(&self) -> impl Display {
        self.accounts
            .deref()
            .iter()
            .map(|a| a.deref())
            .collect::<Vec<&Account>>()
            .with_title()
            .table()
            .display()
            .unwrap()
    }
    pub fn account_remove(&mut self, id: &str) -> Result<(), CliError> {
        let _ = self.account_get(id)?;
        // TODO! Implement stroage connection
        self.accounts.retain(|a| a.id() != id);
        self.account_sort();
        Ok(())
    }
    pub fn account_rename(&mut self, id: &str, name: String) -> Result<(), CliError> {
        let a = self.account_get_mut(id)?;
        // TODO! Implement stroage connection
        a.rename(name);
        self.account_sort();
        Ok(())
    }
    pub fn partner_exist(&self, id: &str) -> bool {
        self.partners.iter().find(|a| a.id == id).is_some()
    }
    pub fn partner_get(&self, id: &str) -> Result<&Partner, CliError> {
        self.partners
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.deref())
            .ok_or(CliError::Error("Partner not found".to_string()))
    }
    pub fn partner_get_mut(&mut self, id: &str) -> Result<&mut Partner, CliError> {
        self.partners
            .iter_mut()
            .find(|a| a.id == id)
            .map(|a| a.deref_mut())
            .ok_or(CliError::Error("Partner not found".to_string()))
    }
    pub fn partner_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.partner_get(&id).is_ok() {
            return Err(CliError::Error("Partner id is taken".to_string()));
        }
        // TODO! Implement stroage connection
        // self.partners.push(Partner { id, name });
        Ok(())
    }
    pub fn partner_get_all(&self) -> impl Display {
        self.partners
            .iter()
            .map(|p| p.deref())
            .collect::<Vec<&Partner>>()
            .with_title()
            .table()
            .display()
            .unwrap()
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
            .map(|n| n.deref())
            .ok_or(CliError::Error("Not found".to_string()))
    }
    pub fn note_get_mut(&mut self, id: &str) -> Result<&mut Note, CliError> {
        self.notes
            .iter_mut()
            .find(|n| n.id == Some(id.to_string()))
            .map(|n| n.deref_mut())
            .ok_or(CliError::Error("Not found".to_string()))
    }
    pub fn note_new(&mut self, id: String) -> Result<(), CliError> {
        // TODO! Implement repository connection
        // self.notes.push(Note::new(Some(id)));
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
        let note = self.note_get_mut(id)?;
        // TODO! Implement repository connection
        // note.set(partner, description, idate, cdate, ddate, net, vat, gross)
        unimplemented!()
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
        let note = self.note_get_mut(id)?;
        // TODO! Implement repository connection
        // note.unset(partner, description, idate, cdate, ddate, net, vat, gross)
        unimplemented!()
    }
    pub fn note_set_transaction(
        &mut self,
        id: &str,
        debit: Account,
        credit: Account,
        amount: f32,
        comment: Option<String>,
    ) -> Result<(), CliError> {
        let note = self.note_get_mut(id)?;
        // TODO! Implement repository connection
        // note.set_transaction(amount, debit, credit, comment)?;
        Ok(())
    }
    pub fn note_filter(&self, id: Option<String>, partner: Option<String>) -> Vec<String> {
        self.notes
            .iter()
            .filter(|n| {
                if let Some(_id) = &id {
                    match &n.id {
                        Some(id) => id.contains(_id),
                        None => false,
                    }
                } else {
                    true
                }
            })
            .filter(|n| {
                if let Some(_partner) = &partner {
                    match &n.partner {
                        Some(partner) => partner.contains(_partner),
                        None => false,
                    }
                } else {
                    true
                }
            })
            .map(|n| n.id.as_deref().unwrap().to_owned())
            .collect::<Vec<String>>()
    }
    pub fn get_ledger(&mut self, month: Option<u32>) -> Result<MonthlySummary, CliError> {
        self.ledger.get(
            self.accounts
                .iter()
                .map(|a| a.deref())
                .collect::<Vec<&Account>>(),
            self.notes.iter().map(|n| n.deref()).collect::<Vec<&Note>>(),
            month,
        )
    }
    pub fn ledger_set_should_update(&mut self) {
        self.ledger.set_should_update();
    }
}

impl Drop for IndexDb {
    fn drop(&mut self) {
        self.save_fs()
    }
}
