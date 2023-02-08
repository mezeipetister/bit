use std::fmt::Display;

use cli_table::{format::Justify, Cell, Style, Table};
use repository::sync::ActionPatch;
use serde::{Deserialize, Serialize};

use crate::actions::BitAction;

#[derive(Serialize, Deserialize, Debug, Default, Eq, Hash, PartialEq, Table, Clone)]
pub struct Account {
    #[table(title = "ID", justify = "Justify::Right")]
    id: String,
    #[table(title = "Name")]
    name: String,
    removed: bool,
}

impl ActionPatch<BitAction> for Account {
    const storage_id: &'static str = "account";
    fn patch(&mut self, action: BitAction, dtime: chrono::DateTime<chrono::Utc>, uid: &str) {
        match action {
            BitAction::AccountCreate { id, name } => {
                self.id = id;
                self.name = name;
            }
            BitAction::AccountRename { name } => self.rename(name),
            BitAction::AccountRemove => self.remove(),
            BitAction::AccountRestore => self.restore(),
            _ => panic!("Just account action can be processed for accounts"),
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = vec![
            vec!["ID".cell(), (&self.id).cell().justify(Justify::Right)],
            vec!["Name".cell(), (&self.name).cell().justify(Justify::Right)],
        ]
        .table()
        .title(vec!["ID".cell().bold(true), "Name".cell().bold(true)]);
        // .bold(true);
        write!(f, "{}", table.display().unwrap())
    }
}

impl Account {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            removed: false,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rename(&mut self, name: String) {
        self.name = name;
    }
    pub fn remove(&mut self) {
        self.removed = true;
    }
    pub fn restore(&mut self) {
        self.removed = false;
    }
}
