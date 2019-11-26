// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use crate::prelude::*;
use crate::repo::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Account1 {
    id: String,
    name: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    is_inverse: bool,
    is_working: bool,
}

impl Account1 {
    pub fn new(id: &str, userid: &str) -> AppResult<Self> {
        Ok(Account1 {
            id: id.trim().to_owned(),
            name: "".into(),
            description: "".into(),
            created_by: userid.into(),
            date_created: Utc::now(),
            is_inverse: false,
            is_working: false,
        })
    }
}

impl Account for Account1 {
    fn set_id(&mut self, id: &str) -> AppResult<()> {
        let id = id.trim();
        self.id = id.into();
        Ok(())
    }
    /// Get account name
    fn get_name(&self) -> String {
        (&self.name).into()
    }
    // TODO: name validation and error!
    /// Set account name, returns AppResult<()>
    fn set_name(&mut self, name: &str) -> AppResult<()> {
        self.name = name.into();
        Ok(())
    }
    /// Get account description, returns AppResult<()>
    fn get_description(&self) -> String {
        (&self.description).into()
    }
    // TODO: Description validation and error!
    /// Set account description, returns AppResult<()>
    fn set_description(&mut self, description: &str) -> AppResult<()> {
        self.description = description.into();
        Ok(())
    }
    /// Get account creation time
    fn get_created_by(&self) -> String {
        (&self.created_by).into()
    }
    /// Get created date
    fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    /// Query is inverse? Return bool
    fn is_inverse(&self) -> bool {
        self.is_inverse
    }
    /// Set account to be inverse, returns AppResult<()>
    fn set_inverse(&mut self, inverse: bool) -> AppResult<()> {
        self.is_inverse = inverse;
        Ok(()) // TODO: Validation
    }
    /// Query account is working
    /// If true, we can work on this account
    fn is_working(&self) -> bool {
        self.is_working
    }
    /// Set account as working, returns AppResult<()>
    fn set_working(&mut self, working: bool) -> AppResult<()> {
        self.is_working = working;
        Ok(()) // TODO: Validation
    }
}

/**
 * StorageObject implementation for Account
 */
// impl storage::StorageObject for Account1 {
//     fn get_id(&self) -> &str {
//         &self.id
//     }
//     // TODO: Fix this one!
//     fn reload(&mut self) -> AppResult<()> {
//         Ok(())
//     }
//     fn get_path(&self) -> Option<&str> {
//         match &self.path {
//             Some(path) => Some(path.as_ref()),
//             None => None,
//         }
//     }
//     fn set_path(&mut self, path: &str) -> AppResult<()> {
//         self.path = Some(path.into());
//         Ok(())
//     }
//     fn get_date_created(&self) -> DateTime<Utc> {
//         self.date_created
//     }
// }
impl StorageObject for Account1 {
    fn get_id(&self) -> &str {
        &self.id
    }
}
