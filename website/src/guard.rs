// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use crate::DataLoad;
use core_lib::user;
use core_lib::user::User;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

pub struct Login {
    userid: String,
    name: String,
    email: String,
}

impl Login {
    pub fn userid(&self) -> &str {
        &self.userid
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}

// TODO: ERROR: An instance of `Cookies` must be dropped before another can be retrieved.
impl<'a, 'r> FromRequest<'a, 'r> for Login {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Login, ()> {
        // Add LOGIN REDIRECT IF PATH EXIST
        let data = request.guard::<State<DataLoad>>()?;
        let users = &mut data.inner().users.lock().unwrap().data;
        let userid = match &request.cookies().get_private("USERID") {
            Some(userid) => userid.value().to_owned(),
            None => {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        match user::get_user_by_id(&mut *users, &userid) {
            Ok(user) => {
                let login = Login {
                    userid: userid,
                    name: user.get_user_name().into(),
                    email: user.get_user_email().into(),
                };
                Outcome::Success(login)
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
