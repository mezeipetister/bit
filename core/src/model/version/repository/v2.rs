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
use crate::model::version::repository::v1::Repository as RepositoryV1;
pub use crate::model::version::transaction::v2::*;

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

impl StorageObject for Repository {
    type ResultType = Repository;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Ok(RepositoryV1::try_from(from)?.into()),
        }
    }
}

impl From<RepositoryV1> for Repository {
    fn from(from: RepositoryV1) -> Self {
        Repository {
            id: from.id,
            name: from.name,
            description: from.description,
            accounts: from.accounts,
            transactions: from
                .transactions
                .iter()
                .map(|t| t.into())
                .collect::<Vec<Transaction>>(),
            assets: from.assets,
            projects: from.projects,
            created_by: from.created_by,
            date_created: from.date_created,
            is_active: from.is_active,
        }
    }
}
