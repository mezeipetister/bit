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

impl Transaction {
    pub fn new(
        id: usize,
        subject: String,
        debit: String,
        credit: String,
        amount: i32,
        date_settlement: NaiveDate,
        created_by: String,
    ) -> Self {
        Transaction {
            id,
            subject,
            debit,
            credit,
            amount,
            date_created: Utc::now(),
            date_settlement,
            created_by,
        }
    }
}

impl Repository {
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn get_transaction_by_id(&self, id: usize) -> AppResult<&Transaction> {
        for item in &self.transactions {
            if item.id == id {
                return Ok(item);
            }
        }
        Err(Error::BadRequest(
            "A megadott tranzakció nem található".to_string(),
        ))
    }
    pub fn add_transaction(
        &mut self,
        subject: String,
        debit: String,
        credit: String,
        amount: i32,
        date_settlement: NaiveDate,
        created_by: String,
    ) -> AppResult<&Transaction> {
        if !self.is_valid_account(&debit) {
            return Err(Error::BadRequest(
                "A megadott debit ID nem könyvelhető".to_string(),
            ));
        }
        if !self.is_valid_account(&credit) {
            return Err(Error::BadRequest(
                "A megadott credit ID nem könyvelhető".to_string(),
            ));
        }
        let transaction = Transaction::new(
            self.transactions.len(),
            subject,
            debit,
            credit,
            amount,
            date_settlement,
            created_by,
        );
        self.transactions.push(transaction.clone());
        self.transactions
            .sort_by(|a, b| a.date_settlement.cmp(&b.date_settlement));
        return if let Some(transaction) = self.transactions.last() {
            Ok(transaction)
        } else {
            Err(Error::InternalError(
                "Error while getting last inserted transaction".to_string(),
            ))
        };
    }
}
