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

use chrono::prelude::*;
use core_lib::model::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SAccount {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_by: String,
    pub date_created: DateTime<Utc>,
    pub is_working: bool,
    pub is_inverse: bool,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SAccountNew {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_working: bool,
    pub is_inverse: bool,
}

impl From<Account> for SAccount {
    fn from(from: Account) -> Self {
        SAccount {
            id: from.get_id().to_string(),
            name: from.get_name().to_string(),
            description: from.get_description().to_string(),
            created_by: from.get_created_by().to_string(),
            date_created: from.get_date_created(),
            is_working: from.get_is_working(),
            is_inverse: from.get_is_inverse(),
            is_active: from.get_is_active(),
        }
    }
}