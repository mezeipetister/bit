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
use core_lib::model::*;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/repository/<repository_id>/account/all")]
pub fn account_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<SAccount>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_accounts()
        .iter()
        .map(|a| (*a).clone().into())
        .collect::<Vec<SAccount>>();
    Ok(StatusOk(res))
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
    data.inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .add_account(account_new.clone())?;
    Ok(StatusOk(account_new.into()))
}

#[get("/repository/<repository_id>/account/<account_id>", rank = 2)]
pub fn account_id_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    account_id: String,
) -> Result<StatusOk<SAccount>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_account_by_id(account_id.clone())?
        .clone();
    Ok(StatusOk(res.into()))
}

#[post(
    "/repository/<repository_id>/account/<account_id>",
    data = "<form>",
    rank = 3
)]
pub fn account_update_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    account_id: String,
    form: Json<SAccount>,
) -> Result<StatusOk<SAccount>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .update_account(
            account_id.clone(),
            form.name.clone(),
            form.description.clone(),
            form.is_working,
            form.is_inverse,
        )?
        .clone();
    Ok(StatusOk(res.into()))
}
