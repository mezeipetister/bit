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

use crate::login::create_token;
use crate::prelude::*;
use crate::DataLoad;
use core_lib::model::User;
use core_lib::password;
use core_lib::user;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FormLogin {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserToken {
    username: String,
    token: String,
}

#[post("/login", data = "<login>")]
pub fn post(
    login: Json<FormLogin>,
    data: State<DataLoad>,
) -> Result<StatusOk<UserToken>, ApiError> {
    // Temp login to admin
    // TODO: Remove this part, vulnerable code
    let username = match &login.username {
        Some(username) => username,
        None => return Err(ApiError::BadRequest("Hiányzó felhasználói név".to_owned())),
    };
    let password = match &login.password {
        Some(password) => password,
        None => return Err(ApiError::BadRequest("Hiányzó jelszó".to_owned())),
    };
    if username == "admin" && password == "admin" {
        return Ok(StatusOk(UserToken {
            username: "Admin".to_owned(),
            token: create_token(&username).unwrap(),
        }));
    }

    // User exist
    if let Ok(user) = &data.inner().users.get_by_id(&username) {
        let hash = user.get(|u| u.get_password_hash().to_owned());
        let res = password::verify_password_from_hash(&password, &hash)?;
        if res {
            return Ok(StatusOk(UserToken {
                username: user.get(|u: &User| u.get_user_name().to_owned()),
                token: create_token(&username).unwrap(),
            }));
        }
    }

    return Err(ApiError::BadRequest("Helytelen belépési adatok".to_owned()));
}

#[derive(Serialize, Deserialize)]
pub struct FormResetPassword {
    email: Option<String>,
}

#[post("/login/reset_password", data = "<form>")]
pub fn reset_password(
    form: Json<FormResetPassword>,
    data: State<DataLoad>,
) -> Result<StatusAccepted<()>, ApiError> {
    let email = match &form.email {
        Some(email) => email,
        None => return Err(ApiError::BadRequest("Hiányzó email cím".to_owned())),
    };

    let user = match user::get_user_by_email(&data.inner().users, email) {
        Ok(user) => user,
        Err(_) => {
            return Err(ApiError::BadRequest(
                "A felhasználó nem található".to_owned(),
            ))
        }
    };

    match &user.update(|u| u.reset_password()) {
        Ok(()) => return Ok(StatusAccepted(())),
        Err(_) => {
            return Err(ApiError::InternalError(
                "Hiba az új jelszó beállításánál".to_owned(),
            ))
        }
    };
}
