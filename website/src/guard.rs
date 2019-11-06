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
use core_lib::user::UserV1;
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

impl<'a, 'r> FromRequest<'a, 'r> for Login {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Login, ()> {
        let data = request.guard::<State<DataLoad>>()?;
        let users = &mut data.inner().users.lock().unwrap().data;
        let userid = match &request.cookies().get_private("USERID") {
            Some(userid) => userid.value().to_owned(),
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };
        // let user: &mut UserV1 = user::get_user_by_email(&mut *users, &form.email);
        let u = user::get_user_by_id(&mut *users, &userid);
        match u {
            Ok(user) => {
                let login = Login {
                    userid: userid,
                    name: user.get_user_name().into(),
                    email: user.get_user_email().into(),
                };
                Outcome::Success(login)
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
            // 1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
        }
    }
}
