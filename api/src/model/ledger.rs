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
pub struct Ledger {
    pub account_id: String,
    pub account_name: String,
    pub is_working: bool,
    pub is_inverse: bool,
    pub is_active: bool,
    pub debit_total: i32,
    pub credit_total: i32,
    pub total: i32,
}

impl Ledger {
    pub fn new(
        account_id: String,
        account_name: String,
        is_working: bool,
        is_inverse: bool,
        is_active: bool,
        debit_total: i32,
        credit_total: i32,
        total: i32,
    ) -> Self {
        Ledger {
            account_id,
            account_name,
            is_working,
            is_inverse,
            is_active,
            debit_total,
            credit_total,
            total,
        }
    }
}
