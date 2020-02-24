// Copyright (C) 2020 Peter Mezei
//
// This file is part of GNStore.
//
// GNStore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// GNStore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNStore.  If not, see <http://www.gnu.org/licenses/>.

use crate::guard::Login;
use crate::prelude::*;
use crate::DataLoad;
use core_lib::model::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/folder/all")]
pub fn folder_all_get(
    _user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<Folder>>, ApiError> {
    let res = data
        .inner()
        .folders
        .into_iter()
        .map(|d| d.get(|c| c.clone()))
        .collect::<Vec<Folder>>();
    Ok(StatusOk(res))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderNew {
    name: String,
    description: String,
}

#[put("/folder/new", data = "<form>")]
pub fn folder_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<FolderNew>,
) -> Result<StatusOk<Folder>, ApiError> {
    let folder_new: Folder = Folder::new(
        user.userid().to_string(),
        form.name.clone(),
        form.description.clone(),
    );
    match data.inner().folders.add_to_storage(folder_new.clone()) {
        Ok(_) => return Ok(StatusOk(folder_new)),
        Err(err) => return Err(err.into()),
    }
}

#[get("/folder/<id>")]
pub fn folder_id_get(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Folder>, ApiError> {
    match data.inner().folders.get_by_id(&id) {
        Ok(folder) => Ok(StatusOk(folder.clone_data())),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderNewName {
    name: String,
}

#[post("/folder/<id>/rename", data = "<form>")]
pub fn folder_rename_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<FolderNewName>,
) -> Result<StatusOk<Folder>, ApiError> {
    match data.inner().folders.get_by_id(&id) {
        Ok(folder) => Ok(StatusOk(folder.update(|f| {
            f.set_name(form.name.clone());
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderNewDescription {
    description: String,
}

#[post("/folder/<id>/redescription", data = "<form>")]
pub fn folder_redescription_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<FolderNewDescription>,
) -> Result<StatusOk<Folder>, ApiError> {
    match data.inner().folders.get_by_id(&id) {
        Ok(folder) => Ok(StatusOk(folder.update(|f| {
            f.set_description(form.description.clone());
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/folder/<id>/remove")]
pub fn folder_remove_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Folder>, ApiError> {
    match data.inner().folders.get_by_id(&id) {
        Ok(folder) => Ok(StatusOk(folder.update(|f| {
            f.remove();
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/folder/<id>/restore")]
pub fn folder_restore_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Folder>, ApiError> {
    match data.inner().folders.get_by_id(&id) {
        Ok(folder) => Ok(StatusOk(folder.update(|f| {
            f.restore();
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}
