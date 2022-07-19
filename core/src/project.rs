use std::{
    collections::{HashMap, HashSet},
    env,
    path::{Path, PathBuf},
};

use crate::{
    fs::Doc,
    ledger::{Account, Ledger, LedgerIndexItem},
    note::Note,
    settings::Settings,
};

#[derive(Default, Debug)]
pub struct Project {
    project_root_path: PathBuf,
    settings: Settings,
    docs: Vec<Doc>,
    accounts_index: HashSet<String>,
    pub accounts: Vec<Account>,
    notes: HashMap<String, Note>,
    pub ledger: Ledger,
}

impl Project {
    pub fn try_init() -> Result<Self, String> {
        // Try to get current working dir
        let current_dir =
            env::current_dir().map_err(|_| "Current working dir does not exist".to_string())?;

        let mut project: Project = Project::default();

        // Try to get project dir & set it
        project.project_root_path = get_project_dir(&current_dir)?;

        // Try to deserialize its contetn
        project.settings = crate::settings::Settings::try_read(&project.project_root_path)?;
        // Init accounts
        project.try_init_accounts()?;
        // Init docs
        project.docs = crate::fs::get_files_recours(
            Path::new(&project.settings.dependencies.docs_path),
            None,
        )?;
        // Init ledger
        project.ledger.init(&project.accounts_index);
        // Init notes
        let note_docs = crate::fs::get_files_recours(
            Path::new(&project.settings.dependencies.notes_path),
            Some("bit"),
        )?;
        for note_doc in note_docs {
            let note: Note = Note::from_file(note_doc.relative_path(), false)?;
            project.ledger.add_note(&note, &project.accounts_index)?;
            project.notes.insert(note.id.clone().unwrap(), note);
        }

        Ok(project)
    }
    fn try_init_accounts(&mut self) -> Result<(), String> {
        // Try load accounts file
        let acc_file = self
            .project_root_path
            .join(&self.settings.dependencies.accounts_path);
        // Check if it exist
        match acc_file.exists() || acc_file.is_file() {
            true => (),
            false => return Err("No account file found!".to_string()),
        }
        let note = Note::from_file(&acc_file, true)?;
        note.accounts.into_iter().for_each(|account| {
            self.accounts_index.insert(account.id.clone());
            self.accounts.push(account);
        });
        Ok(())
    }
}

// Try to get BIT project root path
fn get_project_dir(dir: &Path) -> Result<PathBuf, String> {
    let p = dir.join(".bit");
    match p.exists() && p.is_dir() {
        true => Ok(dir.to_path_buf()),
        false => get_project_dir(
            dir.parent()
                .ok_or("Given directory is not a BIT working directory".to_string())?,
        ),
    }
}
