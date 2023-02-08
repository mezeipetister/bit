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
    fn patch(&mut self, action: BitAction) {
        match action {
            BitAction::AccountRename { name } => self.rename(name),
            BitAction::AccountRemove => self.remove(),
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
    pub fn rename(&mut self, name: String) {
        self.name = name;
    }
    pub fn remove(&mut self) {
        self.removed = true;
    }
}
