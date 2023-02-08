use std::fmt::Display;

use cli_table::{format::Justify, Cell, Style, Table};
use repository::sync::ActionPatch;
use serde::{Deserialize, Serialize};

use crate::actions::BitAction;

#[derive(Serialize, Deserialize, Debug, Default, Eq, Hash, PartialEq, Table, Clone)]
pub struct Partner {
    pub id: String,
    pub name: String,
    pub removed: bool,
}

impl ActionPatch<BitAction> for Partner {
    fn patch(&mut self, action: BitAction, dtime: chrono::DateTime<chrono::Utc>, uid: &str) {
        match action {
            BitAction::PartnerRename { name } => self.set_name(name),
            BitAction::PartnerRemove => self.remove(),
            _ => panic!("Only partner action can be patched to partner"),
        }
    }
}

impl Display for Partner {
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

impl Partner {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            removed: false,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn remove(&mut self) {
        self.removed = true;
    }
    pub fn restore(&mut self) {
        self.removed = true;
    }
}
