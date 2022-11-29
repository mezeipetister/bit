use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sync::{Commit, CommitCandidate, Staging};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    local: Vec<CommitCandidate>,
    remote: Vec<Commit>,
    staging: Staging,
    index: (),
}

impl Database {
    pub fn last_commit_id_remote(&self) -> Uuid {
        unimplemented!()
    }
    pub fn last_commit_id_local(&self) -> Uuid {
        unimplemented!()
    }
}
