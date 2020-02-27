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

use crate::error::Error;
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    /// Repository ID
    /// Automatically generated
    id: String,
    /// Repository name
    /// Created by user
    name: String,
    /// Sort description
    description: String,
    /// Account vector
    accounts: Vec<Account>,
    /// Transaction vector
    transactions: Vec<Transaction>,
    /// Asset vector
    assets: Vec<Asset>,
    /// Project vector
    projects: Vec<Project>,
    /// Created by userid
    created_by: String,
    /// Date created
    date_created: DateTime<Utc>,
    /// Is active
    /// Logical remove
    /// False means its removed
    is_active: bool,
}

impl Repository {
    pub fn new(name: String, description: String, created_by: String) -> Self {
        Repository {
            id: generate_id(5),
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            accounts: Vec::new(),
            transactions: Vec::new(),
            assets: Vec::new(),
            projects: Vec::new(),
            created_by,
            date_created: Utc::now(),
            is_active: true,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
    pub fn add_account(&mut self, account: Account) -> AppResult<()> {
        if let Some(p) = self.accounts.iter().position(|a| a.get_id() == account.id) {
            return Err(Error::BadRequest(
                "A megadott accoutn ID már létezik!".to_string(),
            ));
        }
        self.accounts.push(account);
        self.accounts.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(())
    }
    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn get_account_by_id(&self, id: String) -> AppResult<Account> {
        for account in &self.accounts {
            if account.get_id() == id {
                return Ok(account.clone());
            }
        }
        Err(Error::BadRequest(
            "A megadott ID-val account nem szerepel".to_string(),
        ))
    }
}

impl StorageObject for Repository {
    type ResultType = Repository;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Err(storaget::Error::DeserializeError(
                "document has wrong format".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    id: String,
    name: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    is_working: bool,
    is_inverse: bool,
    is_active: bool,
}

impl Account {
    pub fn new(
        id: String,
        name: String,
        description: String,
        created_by: String,
        is_working: bool,
        is_inverse: bool,
    ) -> Self {
        Account {
            id: id.trim().to_string(),
            name,
            description,
            created_by,
            date_created: Utc::now(),
            is_working,
            is_inverse,
            is_active: true,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_is_working(&self) -> bool {
        self.is_working
    }
    pub fn get_is_inverse(&self) -> bool {
        self.is_inverse
    }
    pub fn set_is_inverse(&mut self, is_inverse: bool) {
        self.is_inverse = is_inverse;
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {}
