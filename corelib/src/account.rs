use std::fmt::Display;

use cli_table::{format::Justify, Cell, Style, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Eq, Hash, PartialEq, Table, Clone)]
pub struct Account {
    #[table(title = "ID", justify = "Justify::Right")]
    pub id: String,
    #[table(title = "Name")]
    pub name: String,
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
        Self { id, name }
    }
}
