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
use core_lib::prelude::AppResult;
use core_lib::user;
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

#[get("/profile")]
pub fn profile_get(user: Login, data: State<DataLoad>) -> Result<StatusOk<Profile>, ApiError> {
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => {
            let profile = usr.get(|user| -> Profile { user.into() });
            return Ok(StatusOk(profile));
        }
        Err(_) => {
            return Err(ApiError::InternalError(
                "A felhasználó nem található.".to_owned(),
            ))
        }
    };
}

#[post("/profile", data = "<form>")]
pub fn profile_post(
    user: Login,
    data: State<DataLoad>,
    form: Json<Profile>,
) -> Result<StatusOk<Profile>, ApiError> {
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => {
            let result = usr.update(|user| -> AppResult<User> {
                user.set_user_name(form.name.clone())?;
                user.set_user_email(form.email.clone())?;
                user.set_user_phone(form.phone.clone())?;
                Ok(user.clone())
            })?;
            return Ok(StatusOk((&result).into()));
        }
        Err(_) => {
            return Err(ApiError::InternalError(
                "A felhasználó nem található.".to_owned(),
            ))
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPassword {
    password1: Option<String>,
    password2: Option<String>,
}

#[post("/profile/new_password", data = "<form>")]
pub fn password_change(
    user: Login,
    data: State<DataLoad>,
    form: Json<NewPassword>,
) -> Result<StatusOk<()>, ApiError> {
    let password1 = match &form.password1 {
        Some(p) => p,
        None => return Err(ApiError::BadRequest("Hiányzó adatmező!".to_owned())),
    };
    let password2 = match &form.password2 {
        Some(p) => p,
        None => return Err(ApiError::BadRequest("Hiányzó adatmező!".to_owned())),
    };
    if password1 != password2 {
        return Err(ApiError::BadRequest(
            "A két jelszó nem egyezik meg egymással".to_owned(),
        ));
    }
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => usr.update(|u| u.set_password(password1.clone()))?,
        Err(_) => return Err(ApiError::InternalError("Azonosítási hiba".to_owned())),
    }
    Ok(StatusOk(()))
}
