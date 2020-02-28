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
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    username: String,
    email: String,
    name: String,
    phone: String,
    date_created: DateTime<Utc>,
    created_by: String,
    // ================
    // Important!
    // ================
    // Only READONLY
    // We do not use it to store any value from form
    // Instead we use direct API call for update customers
    //      ||
    //      \/
    customers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileNew {
    username: String,
    email: String,
    name: String,
    phone: String,
}

impl From<&User> for Profile {
    fn from(user: &User) -> Self {
        Profile {
            username: user.get_id().to_string(),
            email: user.get_user_email().to_string(),
            name: user.get_user_name().to_string(),
            phone: user.get_user_phone().to_string(),
            date_created: user.get_date_created(),
            created_by: user.get_created_by().to_string(),
            customers: user.get_customers(),
        }
    }
}

#[get("/user/all")]
pub fn user_all_get(
    _user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<Profile>>, ApiError> {
    let res = data
        .inner()
        .users
        .into_iter()
        .map(|d| d.get(|c| c.into()))
        .collect::<Vec<Profile>>();
    Ok(StatusOk(res))
}

#[post("/user/new", data = "<form>")]
pub fn user_new_post(
    user: Login,
    data: State<DataLoad>,
    form: Json<ProfileNew>,
) -> Result<StatusOk<Profile>, ApiError> {
    let new_user: User = User::new(
        form.username.clone(),
        form.name.clone(),
        form.email.clone(),
        form.phone.clone(),
        user.userid().to_string(),
    )?;
    // Check if user exist;
    if let Ok(_) = data.inner().users.get_by_id(&new_user.get_id()) {
        return Err(ApiError::BadRequest(
            "A kért user ID már foglalt!".to_owned(),
        ));
    };
    match data.inner().users.add_to_storage(new_user.clone()) {
        Ok(_) => return Ok(StatusOk((&new_user).into())),
        Err(err) => return Err(err.into()),
    }
}

#[get("/user/<id>")]
pub fn user_id_get(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<Profile>, ApiError> {
    match data.inner().users.get_by_id(&id) {
        Ok(user) => Ok(StatusOk((&user.clone_data()).into())),
        Err(_) => Err(ApiError::NotFound),
    }
}
