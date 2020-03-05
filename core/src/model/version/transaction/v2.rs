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

use crate::model::version::transaction::v1;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: usize,
    pub subject: String,
    pub debit: String,
    pub credit: String,
    pub amount: i32,
    pub date_created: DateTime<Utc>,
    pub date_settlement: DateTime<Utc>,
    pub created_by: String,
}

impl From<&v1::Transaction> for Transaction {
    fn from(from: &v1::Transaction) -> Self {
        Transaction {
            id: from.id,
            subject: from.subject.clone(),
            debit: from.debit.clone(),
            credit: from.credit.clone(),
            amount: from.amount,
            date_created: from.date_created,
            date_settlement: DateTime::<Utc>::from_utc(
                DateTime::parse_from_rfc3339(&format!(
                    "{}-{}-{}T00:00:00-00:00",
                    from.date_settlement.year(),
                    from.date_settlement.month(),
                    from.date_settlement.day()
                ))
                .expect("Error while date_settlement convert")
                .naive_utc(),
                Utc,
            ),
            created_by: from.created_by.clone(),
        }
    }
}
