// Copyright (C) 2020 peter
//
// This file is part of BIT.
//
// BIT is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// BIT is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with BIT.  If not, see <http://www.gnu.org/licenses/>.

// use crate::model::*;

pub use crate::model::version::account::v1::Account;
pub use crate::model::version::asset::v1::Asset;
pub use crate::model::version::project::v1::Project;
pub use crate::model::version::transaction::v1::Transaction;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    /// Repository ID
    /// Automatically generated
    pub id: String,
    /// Repository name
    /// Created by user
    pub name: String,
    /// Sort description
    pub description: String,
    /// Account vector
    pub accounts: Vec<Account>,
    /// Transaction vector
    pub transactions: Vec<Transaction>,
    /// Asset vector
    pub assets: Vec<Asset>,
    /// Project vector
    pub projects: Vec<Project>,
    /// Created by userid
    pub created_by: String,
    /// Date created
    pub date_created: DateTime<Utc>,
    /// Is active
    /// Logical remove
    /// False means its removed
    pub is_active: bool,
}

impl Default for Repository {
    fn default() -> Self {
        Repository {
            id: String::default(),
            name: String::default(),
            description: String::default(),
            accounts: Vec::new(),
            transactions: Vec::new(),
            assets: Vec::new(),
            projects: Vec::new(),
            created_by: String::default(),
            date_created: Utc::now(),
            is_active: true,
        }
    }
}

impl VecPackMember for Repository {
    fn get_id(&self) -> &str {
        &self.id
    }
    // fn try_from(from: &str) -> StorageResult<Self::ResultType> {
    //     match deserialize_object(from) {
    //         Ok(res) => Ok(res),
    //         Err(_) => Err(storaget::Error::DeserializeError(
    //             "document has wrong format".to_string(),
    //         )),
    //     }
    // }
}
