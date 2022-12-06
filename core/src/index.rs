use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    core::{Commit, CommitCandidate, Staging},
    prelude::BitResult,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Index {
    ledger: (),
    pub commit_count: i32,
}

impl Index {
    pub fn build(
        mut self,
        remote: Vec<Commit>,
        local: Vec<CommitCandidate>,
        staging: Staging,
    ) -> BitResult<Self> {
        self.commit_count = (remote.len() + local.len()) as i32;
        Ok(self)
    }
}
