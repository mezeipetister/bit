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
use crate::model::*;
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

#[get("/repository/<repository_id>/account/all")]
pub fn account_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<SAccount>>, ApiError> {
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repository) => Ok(StatusOk(repository.get(|f| {
            f.get_accounts()
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SAccount>>()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[put("/repository/<repository_id>/account/new", data = "<form>")]
pub fn account_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<SAccountNew>,
    repository_id: String,
) -> Result<StatusOk<SAccount>, ApiError> {
    let account_new = Account::new(
        form.id.clone(),
        form.name.clone(),
        form.description.clone(),
        user.userid().to_string(),
        form.is_working,
        form.is_inverse,
    );
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repo) => {
            repo.update(|r| -> AppResult<()> {
                r.add_account(account_new.clone())?;
                Ok(())
            })?;
            Ok(StatusOk(account_new.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

// #[post("/repository/<id>/remove")]
// pub fn repository_remove_post(
//     _user: Login,
//     data: State<DataLoad>,
//     id: String,
// ) -> Result<StatusOk<SRepositoryShort>, ApiError> {
//     match data.inner().repositories.get_by_id(&id) {
//         Ok(repository) => Ok(StatusOk(repository.update(|f| {
//             f.remove();
//             f.clone().into()
//         }))),
//         Err(_) => Err(ApiError::NotFound),
//     }
// }

#[get("/repository/<repository_id>/account/<account_id>", rank = 2)]
pub fn account_id_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    account_id: String,
) -> Result<StatusOk<SAccount>, ApiError> {
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(rep) => Ok(StatusOk(
            rep.get(|r| r.get_account_by_id(account_id.clone()))?.into(),
        )),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post(
    "/repository/<repository_id>/account/<account_id>",
    data = "<form>",
    rank = 3
)]
pub fn repository_update_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    account_id: String,
    form: Json<SAccount>,
) -> Result<StatusOk<SAccount>, ApiError> {
    match data.inner().repositories.get_by_id(&repository_id) {
        Ok(repository) => Ok(StatusOk(repository.update(|f| {
            f.set_name(form.name.to_string());
            f.set_description(form.description.to_string());
            f.clone().into()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

// #[post("/repository/<id>/restore")]
// pub fn repository_restore_post(
//     _user: Login,
//     data: State<DataLoad>,
//     id: String,
// ) -> Result<StatusOk<SRepositoryShort>, ApiError> {
//     match data.inner().repositories.get_by_id(&id) {
//         Ok(repository) => Ok(StatusOk(repository.update(|f| {
//             f.restore();
//             f.clone().into()
//         }))),
//         Err(_) => Err(ApiError::NotFound),
//     }
// }
