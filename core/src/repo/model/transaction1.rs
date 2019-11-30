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

use crate::error::Error;
use crate::prelude::*;
use crate::repo::Account;
use crate::repo::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::{Storage, StorageObject};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction1 {
    id: String,
    subject: String,
    debit: String,
    credit: String,
    amount: u32,
    date_created: NaiveDateTime,
    date_settlement: NaiveDate,
    // date_posting: Date<Utc>,
    // Duedate should be a part of event details
    // duedate: Date<Utc>,
    created_by: String,
    // event_id: String,
    // commit_id: String,
    // If commit accepted, is_accepted field is true.
    // Only true when it is a part of the ledger.
    // At this point, any event transaction will be a part
    // of ledger immediately.
    // is_ledger_memeber: bool,
}

impl Transaction1 {
    pub fn new<T>(
        subject: String,
        debit: String,
        credit: String,
        amount: u32,
        date_settlement: NaiveDate,
        created_by: String,
        accounts: &Storage<T>,
    ) -> AppResult<Self>
    where
        T: Account,
    {
        let debit_account = accounts.get_by_id(&debit);
        let credit_account = accounts.get_by_id(&credit);
        if debit_account.is_err() {
            return Err(Error::InternalError("Debit account unknown".to_owned()));
        }
        if credit_account.is_err() {
            return Err(Error::InternalError("Credit account unknown".to_owned()));
        }
        if !debit_account.unwrap().get(|a| a.is_working()) {
            return Err(Error::InternalError(
                "Debit account is not a working kind".to_owned(),
            ));
        }
        if !credit_account.unwrap().get(|a| a.is_working()) {
            return Err(Error::InternalError(
                "Credit account is not a working kind".to_owned(),
            ));
        }
        Ok(Transaction1 {
            id: create_new_id()?,
            subject,
            debit,
            credit,
            amount,
            date_created: Utc::now().naive_utc(),
            date_settlement,
            created_by,
        })
    }
}

impl Transaction for Transaction1 {
    fn get_subject(&self) -> String {
        self.subject.clone()
    }
    fn get_debit_credit(&self) -> (String, String) {
        (self.debit.clone(), self.credit.clone())
    }
    fn get_debit(&self) -> String {
        self.debit.clone()
    }
    fn get_credit(&self) -> String {
        self.credit.clone()
    }
    fn get_amount(&self) -> u32 {
        self.amount
    }
    fn get_date_created(&self) -> NaiveDateTime {
        self.date_created
    }
    fn get_date_settlement(&self) -> NaiveDate {
        self.date_settlement
    }
    fn get_created_by(&self) -> String {
        self.created_by.clone()
    }
}

impl StorageObject for Transaction1 {
    fn get_id(&self) -> &str {
        &self.id
    }
}
