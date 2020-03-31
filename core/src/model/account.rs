// Copyright (C) 2020 Peter Mezei
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
use crate::model::*;
use crate::prelude::*;
use chrono::prelude::*;

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

impl Repository {
    pub fn add_account(&mut self, account: Account) -> AppResult<&Account> {
        if let Some(_) = self
            .accounts
            .iter()
            .position(|a| a.get_id() == account.get_id())
        {
            return Err(Error::BadRequest(
                "A megadott accoutn ID már létezik!".to_string(),
            ));
        }
        // Check if account id contains just numbers
        if !account.get_id().chars().all(char::is_numeric) {
            return Err(Error::BadRequest(
                "A számla száma csak számokat tartalmazhat!".to_string(),
            ));
        }
        self.accounts.push(account);
        self.accounts.sort_by(|a, b| a.get_id().cmp(&b.get_id()));
        if let Some(account) = self.accounts.first() {
            return Ok(&account);
        }
        Err(Error::InternalError(
            "Cannot get the last added account ref".to_string(),
        ))
    }
    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn get_account_by_id(&self, id: String) -> AppResult<&Account> {
        for account in &self.accounts {
            if account.get_id() == id {
                return Ok(&account);
            }
        }
        Err(Error::BadRequest(
            "A megadott ID-val account nem szerepel".to_string(),
        ))
    }
    pub fn is_valid_account(&self, id: &str) -> bool {
        for account in &self.accounts {
            if account.get_id() == id && account.get_is_working() && account.get_is_active() {
                return true;
            }
        }
        false
    }
    pub fn update_account(
        &mut self,
        account_id: String,
        name: String,
        description: String,
        is_working: bool,
        is_inverse: bool,
    ) -> AppResult<&Account> {
        for account in &mut self.accounts {
            if account.get_id() == account_id {
                account.name = name;
                account.description = description;
                account.is_working = is_working;
                account.is_inverse = is_inverse;
                return Ok(account);
            }
        }
        Err(Error::BadRequest(
            "A számla azonosító nem található".to_string(),
        ))
    }
}
