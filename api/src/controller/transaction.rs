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
use rocket::response::NamedFile;
use rocket::Data;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[get("/repository/<repository_id>/transaction/all")]
pub fn transaction_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<apiSchema::Transaction>>, ApiError> {
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repository) => Ok(StatusOk(repository.get(|f: &Repository| {
            f.get_transactions()
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<apiSchema::Transaction>>()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[put("/repository/<repository_id>/transaction/new", data = "<form>")]
pub fn transaction_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<apiSchema::TransactionNew>,
    repository_id: String,
) -> Result<StatusOk<apiSchema::Transaction>, ApiError> {
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repo) => {
            let transaction = repo.update(|r: &mut Repository| -> AppResult<Transaction> {
                r.add_transaction(
                    form.subject.clone(),
                    form.debit.clone(),
                    form.credit.clone(),
                    form.amount.clone(),
                    form.date_settlement.clone(),
                    user.userid().to_string(),
                )
            })?;
            Ok(StatusOk(transaction.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

// #[get("/repository/<repository_id>/account/<account_id>", rank = 2)]
// pub fn account_id_get(
//     _user: Login,
//     data: State<DataLoad>,
//     repository_id: String,
//     account_id: String,
// ) -> Result<StatusOk<SAccount>, ApiError> {
//     match data.inner().repositories.get_by_id(&repository_id) {
//         Ok(rep) => Ok(StatusOk(
//             rep.get(|r| r.get_account_by_id(account_id.clone()))?.into(),
//         )),
//         Err(_) => Err(ApiError::NotFound),
//     }
// }

// #[post(
//     "/repository/<repository_id>/account/<account_id>",
//     data = "<form>",
//     rank = 3
// )]
// pub fn account_update_post(
//     _user: Login,
//     data: State<DataLoad>,
//     repository_id: String,
//     account_id: String,
//     form: Json<SAccount>,
// ) -> Result<StatusOk<SAccount>, ApiError> {
//     match data.inner().repositories.get_by_id(&repository_id) {
//         Ok(repository) => Ok(StatusOk(
//             repository
//                 .update(|r| {
//                     r.update_account(
//                         account_id.clone(),
//                         form.name.clone(),
//                         form.description.clone(),
//                         form.is_working,
//                         form.is_inverse,
//                     )
//                 })?
//                 .into(),
//         )),
//         Err(_) => Err(ApiError::NotFound),
//     }
// }
