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
use chrono::prelude::*;
use core_lib::model::*;
use rocket::response::NamedFile;
use rocket::Data;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[get("/document/<folder_id>/all")]
pub fn document_all_get(
    _user: Login,
    data: State<DataLoad>,
    folder_id: String,
) -> Result<StatusOk<Vec<Document>>, ApiError> {
    let res = data
        .inner()
        .documents
        .into_iter()
        .filter(|d| d.get(|c| c.get_folder() == &folder_id && c.is_active()))
        .map(|d| d.get(|c| c.clone()))
        .collect::<Vec<Document>>();
    Ok(StatusOk(res))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentNew {
    reference: String,
    title: String,
    description: String,
}

#[put("/document/<folder_id>/new", data = "<form>")]
pub fn document_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<DocumentNew>,
    folder_id: String,
) -> Result<StatusOk<Document>, ApiError> {
    let document_new: Document = Document::new(
        user.userid().to_string(),
        form.reference.clone(),
        folder_id.clone(),
        form.title.clone(),
        form.description.clone(),
    );
    match data.inner().documents.add_to_storage(document_new.clone()) {
        Ok(_) => return Ok(StatusOk(document_new)),
        Err(err) => return Err(err.into()),
    }
}

#[get("/document/<id>")]
pub fn document_id_get(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Document>, ApiError> {
    match data.inner().documents.get_by_id(&id) {
        Ok(document) => Ok(StatusOk(document.clone_data())),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/document/<id>/remove")]
pub fn document_remove_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Document>, ApiError> {
    match data.inner().documents.get_by_id(&id) {
        Ok(document) => Ok(StatusOk(document.update(|f| {
            f.remove();
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/document/<id>", data = "<form>")]
pub fn document_update_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<Document>,
) -> Result<StatusOk<Document>, ApiError> {
    match data.inner().documents.get_by_id(&id) {
        Ok(document) => Ok(StatusOk(document.update(|f| {
            f.set_title(form.get_title().to_string());
            f.set_description(form.get_description().to_string());
            f.set_reference(form.get_reference().to_string());
            f.set_due_date(form.get_due_date());
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/document/<id>/restore")]
pub fn document_restore_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Document>, ApiError> {
    match data.inner().documents.get_by_id(&id) {
        Ok(document) => Ok(StatusOk(document.update(|f| {
            f.restore();
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentNewDuedate {
    due_date: Option<DateTime<Utc>>,
}

#[post("/document/<id>/due_date", data = "<form>")]
pub fn document_duedate_post(
    _user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<DocumentNewDuedate>,
) -> Result<StatusOk<Document>, ApiError> {
    match data.inner().documents.get_by_id(&id) {
        Ok(document) => Ok(StatusOk(document.update(|f| {
            f.set_due_date(form.due_date);
            f.clone()
        }))),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post(
    "/document/<id>/upload_file",
    format = "application/pdf",
    data = "<file>"
)]
pub fn document_upload_file_post(
    _user: Login,
    file: Data,
    id: String,
    data: State<DataLoad>,
) -> Result<StatusOk<Document>, ApiError> {
    let fname = format!("data/file/{}.pdf", &id);
    std::fs::create_dir_all("data/file");
    let path = Path::new(&fname);
    std::fs::File::create(path);
    match file.stream_to_file(&path) {
        Ok(_) => match data.inner().documents.get_by_id(&id) {
            Ok(document) => Ok(StatusOk(document.update(|d| {
                d.set_file(Some(id.clone()));
                d.clone()
            }))),
            Err(_) => Err(ApiError::NotFound),
        },
        Err(msg) => Err(ApiError::InternalError(format!("{}", msg))),
    }
}

#[get("/file/<id>")]
pub fn document_file_get(_user: Login, id: String) -> Result<NamedFile, ApiError> {
    match NamedFile::open(&format!("data/file/{}.pdf", id)) {
        Ok(f) => Ok(f),
        Err(msg) => Err(ApiError::InternalError(format!("{}", msg))),
    }
}
