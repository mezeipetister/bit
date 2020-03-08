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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub account: String,
    pub account_clearing: String,
    pub value: u32,
    pub date_activated: NaiveDate,
    pub depreciation_key: f32,
    pub residual_value: u32,
    pub date_created: DateTime<Utc>,
    pub created_by: String,
    pub is_active: bool,
}
