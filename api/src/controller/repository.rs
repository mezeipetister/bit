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
use rocket::response::NamedFile;
use rocket::Data;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[get("/repository/all")]
pub fn repository_all_get(
    _user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<SRepositoryShort>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .into_iter()
        .filter(|d| d.get(|r| r.get_is_active()))
        .map(|d| d.get(|c| c.clone().into()))
        .collect::<Vec<SRepositoryShort>>();
    Ok(StatusOk(res))
}

#[put("/repository/new", data = "<form>")]
pub fn repository_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<SRepositoryNew>,
) -> Result<StatusOk<SRepositoryShort>, ApiError> {
    let repository_new = Repository::new(
        form.name.clone(),
        form.description.clone(),
        user.userid().to_string(),
    );
    match data
        .inner()
        .repositories
        .add_to_storage(repository_new.clone())
    {
        Ok(_) => return Ok(StatusOk(repository_new.into())),
        Err(err) => return Err(err.into()),
    }
}

#[post("/repository/<id>/remove")]
pub fn repository_remove_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<SRepositoryShort>, ApiError> {
    match data.inner().repositories.get_by_id(&id) {
        Ok(repository) => Ok(StatusOk(repository.update(|f| {
            f.remove();
            f.clone().into()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[get("/repository/<id>")]
pub fn repository_id_get(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<SRepositoryShort>, ApiError> {
    match data.inner().repositories.get_by_id(&id) {
        Ok(rep) => Ok(StatusOk(rep.clone_data().into())),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/repository/<id>", data = "<form>")]
pub fn repository_update_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<SRepositoryNew>,
) -> Result<StatusOk<SRepositoryShort>, ApiError> {
    match data.inner().repositories.get_by_id(&id) {
        Ok(repository) => Ok(StatusOk(repository.update(|f| {
            f.set_name(form.name.to_string());
            f.set_description(form.description.to_string());
            f.clone().into()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/repository/<id>/restore")]
pub fn repository_restore_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<SRepositoryShort>, ApiError> {
    match data.inner().repositories.get_by_id(&id) {
        Ok(repository) => Ok(StatusOk(repository.update(|f| {
            f.restore();
            f.clone().into()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}
