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
pub struct Transaction {
    pub id: usize,
    pub subject: String,
    pub debit: String,
    pub credit: String,
    pub amount: i32,
    pub date_created: DateTime<Utc>,
    pub date_settlement: NaiveDate,
    pub created_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionNew {
    pub subject: String,
    pub debit: String,
    pub credit: String,
    pub amount: i32,
    pub date_settlement: NaiveDate,
}

impl From<model::Transaction> for Transaction {
    fn from(from: model::Transaction) -> Self {
        Transaction {
            id: from.id,
            subject: from.subject,
            debit: from.debit,
            credit: from.credit,
            amount: from.amount,
            date_created: from.date_created,
            date_settlement: from.date_settlement,
            created_by: from.created_by,
        }
    }
}

impl From<&model::Transaction> for Transaction {
    fn from(from: &model::Transaction) -> Self {
        Transaction {
            id: from.id,
            subject: from.subject.to_string(),
            debit: from.debit.to_string(),
            credit: from.credit.to_string(),
            amount: from.amount,
            date_created: from.date_created,
            date_settlement: from.date_settlement,
            created_by: from.created_by.to_string(),
        }
    }
}
