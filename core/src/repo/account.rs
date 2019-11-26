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
use chrono::prelude::*;
use storaget::*;

pub trait Account: StorageObject {
    /// Set account ID, returns AppResult<()>
    fn set_id(&mut self, id: &str) -> AppResult<()>;
    /// Get account name
    fn get_name(&self) -> &str;
    /// Set account name, returns AppResult<()>
    fn set_name(&mut self, name: &str) -> AppResult<()>;
    /// Get account description, returns AppResult<()>
    fn get_description(&self) -> &str;
    /// Set account description, returns AppResult<()>
    fn set_description(&mut self, description: &str) -> AppResult<()>;
    /// Get account creation time
    fn get_created_by(&self) -> &str;
    /// Get created date
    fn get_date_created(&self) -> DateTime<Utc>;
    /// Query is inverse? Return bool
    fn is_inverse(&self) -> bool;
    /// Set account to be inverse, returns AppResult<()>
    fn set_inverse(&mut self, inverse: bool) -> AppResult<()>;
    /// Query account is working
    /// If true, we can work on this account
    fn is_working(&self) -> bool;
    /// Set account as working, returns AppResult<()>
    fn set_working(&mut self, working: bool) -> AppResult<()>;
}

pub fn accounts_set_order_by_id<T>(accounts: &mut Vec<T>)
where
    T: Account,
{
    accounts.sort_by(|a, b| a.get_id().partial_cmp(&b.get_id()).unwrap());
}
