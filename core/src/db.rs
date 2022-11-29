use serde::{Deserialize, Serialize};

use crate::sync::{Commit, Staging};

#[derive(Serialize, Deserialize, Debug)]
pub struct Db {
    local: Vec<Commit>,
    remote: Vec<Commit>,
    staging: Staging,
    index: (),
}
