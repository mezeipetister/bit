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

use crate::guard::Login;
use crate::model as apiSchema;
use crate::prelude::*;
use crate::DataLoad;
use chrono::prelude::*;
use core_lib::model::*;
use core_lib::prelude::AppResult;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::Data;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[derive(FromForm)]
pub struct Filter {
    till: String,
}

fn parse_date(dt: &str) -> AppResult<NaiveDate> {
    match NaiveDate::parse_from_str(dt, "%Y-%m-%d") {
        Ok(dt) => Ok(dt),
        Err(_) => Err(core_lib::Error::BadRequest(
            "Rossz dátum formátum!".to_string(),
        )),
    }
}

#[get("/repository/<repository_id>/ledger?<filter..>")]
pub fn ledger_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    filter: Form<Filter>,
) -> Result<StatusOk<Vec<apiSchema::Ledger>>, ApiError> {
    let till = parse_date(&filter.till)?;
    // First get ledger vector;
    // TODO: Rename ledger to something like LedgerItem
    let mut ledger: Vec<apiSchema::Ledger> =
        match data.inner().repositories.get_by_id(&repository_id) {
            Ok(repository) => repository.get(|f| {
                f.get_accounts()
                    .iter()
                    .map(|a: &Account| {
                        // Create a new ledger item with account
                        // details and 0 initial values
                        apiSchema::Ledger::new(
                            a.get_id().to_string(),
                            a.get_name().to_string(),
                            a.get_is_working(),
                            a.get_is_inverse(),
                            a.get_is_active(),
                            0,
                            0,
                            0,
                        )
                    })
                    .collect::<Vec<apiSchema::Ledger>>()
            }),
            Err(_) => Vec::new(),
        };

    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repository) => repository.update(|f: &mut Repository| {
            for transaction in f.get_transactions().into_iter() {
                // Apply date filter
                if transaction.date_settlement <= till {
                    for ledger_item in &mut ledger {
                        if transaction.debit == ledger_item.account_id {
                            ledger_item.debit_total += transaction.amount;
                        }
                        if transaction.credit == ledger_item.account_id {
                            ledger_item.credit_total += transaction.amount;
                        }
                    }
                }
            }
        }),
        Err(_) => return Err(ApiError::NotFound),
    }

    for item in &mut ledger {
        item.total = item.debit_total - item.credit_total;
    }

    // Return ledger vector
    Ok(StatusOk(ledger))
}

#[derive(FromForm)]
pub struct StatFilter {
    account: String,
}

#[get("/repository/<repository_id>/ledger/stat?<filter..>")]
pub fn ledger_stat_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    filter: Form<StatFilter>,
) -> Result<StatusOk<[Option<i32>; 12]>, ApiError> {
    // Init result array
    let mut result: [Option<i32>; 12] = [None; 12];

    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repository) => repository.update(|f: &mut Repository| {
            for transaction in f.get_transactions() {
                // Filter just the current year events
                if transaction.date_settlement.year() != Utc::now().naive_utc().year() {
                    continue;
                }
                let month = (transaction.date_settlement.month() - 1) as usize;
                if transaction.debit.starts_with(&filter.account) {
                    match result[month] {
                        Some(value) => result[month] = Some(value + transaction.amount),
                        None => result[month] = Some(transaction.amount),
                    }
                }
                if transaction.credit.starts_with(&filter.account) {
                    match result[month] {
                        Some(value) => result[month] = Some(value - transaction.amount),
                        None => result[month] = Some(transaction.amount * -1),
                    }
                }
            }
        }),
        Err(_) => return Err(ApiError::NotFound),
    }

    Ok(StatusOk(result))
}
