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
use crate::model as ApiSchema;
use crate::prelude::*;
use crate::DataLoad;
use core_lib::model::*;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/repository/<repository_id>/project/all")]
pub fn project_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<ApiSchema::Project>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_projects()
        .iter()
        .map(|a| (*a).clone().into())
        .collect::<Vec<ApiSchema::Project>>();
    Ok(StatusOk(res))
}

#[put("/repository/<repository_id>/project/new", data = "<form>")]
pub fn project_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<ApiSchema::ProjectNew>,
    repository_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let project_new = Project::new(
        form.name.clone(),
        form.description.clone(),
        user.userid().to_string(),
    );
    let p = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .add_project(
            project_new.name,
            project_new.description,
            user.userid().to_string(),
        )?;
    Ok(StatusOk(p.into()))
}

#[get("/repository/<repository_id>/project/<project_id>", rank = 2)]
pub fn project_id_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_project_by_id(&project_id)?
        .into();
    Ok(StatusOk(res))
}

#[post(
    "/repository/<repository_id>/project/<project_id>",
    data = "<form>",
    rank = 3
)]
pub fn project_update_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
    form: Json<ApiSchema::ProjectUpdate>,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .update_project(
            &project_id,
            form.name.clone(),
            form.description.clone(),
            form.is_enabled,
        )?;
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/remove", rank = 3)]
pub fn project_remove_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .remove_project_by_id(&project_id)?;
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/enable", rank = 3)]
pub fn project_enable_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .enable_project(&project_id)?;
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/disable", rank = 3)]
pub fn project_disable_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .disable_project(&project_id)?;
    Ok(StatusOk(res.into()))
}
