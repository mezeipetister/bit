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
use core_lib::model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub depreciation_last_day_value: u32,
    pub depreciation_last_day: NaiveDate,
    pub depreciation_daily_value: u32,
    pub depreciation_monthly: Vec<(NaiveDate, u32, u32)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetNew {
    pub name: String,
    pub description: String,
    pub account: String,
    pub account_clearing: String,
    pub value: u32,
    pub date_activated: NaiveDate,
    pub depreciation_key: f32,
    pub residual_value: u32,
}

impl From<model::Asset> for Asset {
    fn from(f: model::Asset) -> Self {
        Asset {
            depreciation_monthly: f.depreciation_monthly_vector(),
            depreciation_last_day_value: f.depreciation_last_day_value(),
            depreciation_last_day: f.depreciation_last_day(),
            depreciation_daily_value: f.depreciation_daily_value(),
            id: f.id,
            name: f.name,
            description: f.description,
            account: f.account,
            account_clearing: f.account_clearing,
            value: f.value,
            date_activated: f.date_activated,
            depreciation_key: f.depreciation_key,
            residual_value: f.residual_value,
            date_created: f.date_created,
            created_by: f.created_by,
            is_active: f.is_active,
        }
    }
}
