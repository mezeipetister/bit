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
use repository::sync::{DocRefVec, Document, IndexExt, Mode, Repository};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use uuid::Uuid;

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
        let _ = self.repository.proceed_pull::<BitAction>(&mut self.inner)?;
        Ok(())
    }
    pub fn push(&mut self) -> Result<(), IndexError> {
        let _ = self.repository.proceed_push::<BitAction>(&mut self.inner)?;
        Ok(())
    }
    pub fn commit(&mut self, comment: String) -> Result<(), String> {
        let res = self.repository.commit::<BitAction>(comment)?;
        Ok(())
    }
    fn save_fs(&self) {
        binary_update(path_helper::index(&self.ctx), &self.inner)
            .expect("Error writing bit db to fs");
    }
    pub fn print_document(&self, doc_id: Uuid) -> Result<String, CliError> {
        let res = self
            .repository
            .get_doc::<BitAction>(doc_id)
            .map_err(|e| CliError::Error(e))?;
        Ok(res.print())
    }
    pub fn db_emptyindex(&mut self) -> Result<(), CliError> {
        self.inner.reset_docrefs().map_err(|e| CliError::Error(e))
    }
    pub fn db_reindex(&mut self) -> Result<(), CliError> {
        self.repository
            .reset_index(&mut self.inner)
            .map_err(|e| CliError::Error(e))
    }
    pub fn account_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        match self.account_get(&id) {
            Ok(_) => return Err(CliError::Error("Account id is taken".to_string())),
            Err(_) => {
                let aob = self.accounts.create_init_aob(
                    BitAction::AccountCreate { id, name },
                    "mezeipetister".to_string(),
                );
                self.repository.add_aob(aob, &mut self.inner).unwrap();
                self.account_sort();
                Ok(())
            }
        }
    }
    pub fn account_history(&self, id: &str) -> Result<String, CliError> {
        let account = self
            .accounts
            .iter()
            .find(|a| a.deref().id() == id)
            .ok_or(CliError::Error("Account not found".to_string()))?;
        self.print_document(account.object_id)
    }
    fn account_patch(&mut self, id: &str, patch: BitAction) -> Result<(), CliError> {
        let account = self
            .accounts
            .iter()
            .find(|a| a.deref().id() == id)
            .ok_or(CliError::Error("Account not found".to_string()))?;
        let aob = account
            .create_aob(&self.repository, patch)
            .map_err(|e| CliError::Error(e))?;
        self.repository.add_aob(aob, &mut self.inner).unwrap();
        self.account_sort();
        Ok(())
    }
    pub fn account_remove(&mut self, id: &str) -> Result<(), CliError> {
        self.account_patch(id, BitAction::AccountRemove)?;
        Ok(())
    }
    pub fn account_restore(&mut self, id: &str) -> Result<(), CliError> {
        self.account_patch(id, BitAction::AccountRestore)?;
        Ok(())
    }
    pub fn account_rename(&mut self, id: &str, name: String) -> Result<(), CliError> {
        self.account_patch(id, BitAction::AccountRename { name })?;
        Ok(())
    }
    fn partner_patch(&mut self, id: &str, patch: BitAction) -> Result<(), CliError> {
        let partner = self
            .partners
            .iter()
            .find(|a| a.deref().id == id)
            .ok_or(CliError::Error("Partner not found".to_string()))?;
        let aob = partner
            .create_aob(&self.repository, patch)
            .map_err(|e| CliError::Error(e))?;
        self.repository.add_aob(aob, &mut self.inner).unwrap();
        self.account_sort();
        Ok(())
    }
    pub fn partner_add(&mut self, id: String, name: String) -> Result<(), CliError> {
        if self.partner_get(&id).is_ok() {
            return Err(CliError::Error("Partner id is taken".to_string()));
        }
        let aob = self.partners.create_init_aob(
            BitAction::PartnerCreate { id, name },
            "mezeipetister".to_string(),
        );
        self.repository.add_aob(aob, &mut self.inner).unwrap();
        self.account_sort();
        Ok(())
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
    pub fn note_add(&mut self, id: String) -> Result<(), CliError> {
        if self.note_get(&id).is_ok() {
            return Err(CliError::Error("Note id is taken".to_string()));
        }
        let aob = self
            .notes
            .create_init_aob(BitAction::NoteCreate { id }, "mezeipetister".to_string());
        self.repository.add_aob(aob, &mut self.inner).unwrap();
        Ok(())
    }
    fn note_patch(&mut self, id: &str, patch: BitAction) -> Result<(), CliError> {
        let note = self
            .notes
            .iter()
            .find(|a| a.deref().id.as_deref() == Some(id))
            .ok_or(CliError::Error("Note not found".to_string()))?;
        let aob = note
            .create_aob(&self.repository, patch)
            .map_err(|e| CliError::Error(e))?;
        self.repository.add_aob(aob, &mut self.inner).unwrap();
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
        if self.note_get(id).is_err() {
            return Err(CliError::Error("Note not found".to_string()));
        }

        self.note_patch(
            id,
            BitAction::NoteSet {
                partner: partner.map(|p| p.id),
                description,
                idate,
                cdate,
                ddate,
                net,
                vat,
                gross,
            },
        )?;

        Ok(())
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
        if self.note_get(id).is_err() {
            return Err(CliError::Error("Note not found".to_string()));
        }
        // TODO! Implement repository connection
        self.note_patch(
            id,
            BitAction::NoteUnset {
                partner,
                description,
                idate,
                cdate,
                ddate,
                net,
                vat,
                gross,
            },
        )?;
        Ok(())
    }
    pub fn note_set_transaction(
        &mut self,
        id: &str,
        debit: Account,
        credit: Account,
        amount: f32,
        comment: Option<String>,
    ) -> Result<(), CliError> {
        if self.note_get(id).is_err() {
            return Err(CliError::Error("Note not found".to_string()));
        }

        self.note_patch(
            id,
            BitAction::NoteSetTransaction {
                amount,
                debit: debit.id().to_string(),
                credit: credit.id().to_string(),
                comment,
            },
        )?;
        Ok(())
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
        self.ledger = Ledger::default();
        Ok(())
    }

    fn sync_doc(&mut self, doc: &Document<Self::ActionType>) -> Result<(), String> {
        let res = match doc.storage_id.as_str() {
            "account" => self.accounts.sync_with_doc(doc),
            "note" => self.notes.sync_with_doc(doc),
            "partner" => self.partners.sync_with_doc(doc),
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
    pub fn account_get_all(&mut self) -> impl Display {
        self.account_sort();
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
        self.account_sort();
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

#[derive(Debug)]
pub struct ServerDb {
    repository: Repository,
}

impl ServerDb {
    pub fn load() -> Result<Self, String> {
        let ctx = Context::new().map_err(|e| e.to_string())?;
        let repository = Repository::load(repository::sync::Context::init(
            ctx.bitdir_path().join("sync"),
            "demo".to_string(), // TODO! Implement UID
        ))?;
        Ok(Self { repository })
    }
    pub fn start_server(self) -> Result<(), String> {
        match self.repository.is_server_mode() {
            true => {
                println!(
                    "Server started at address: {}",
                    self.repository
                        .get_server_address()
                        .unwrap_or(&"-".to_string())
                );
                let _ = self.repository.start_server::<BitAction>().serve()?;
            }
            false => return Err("Repository is not in server mode!".to_string()),
        }
        Ok(())
    }
}
