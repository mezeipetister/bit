use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    ledger: (),
}

impl Index {}
