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

use crate::login::verify_token;
use crate::DataLoad;
use core_lib::user;
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

// TODO: Rewrite to JWT like, using Yew we do not have access to Cookies.
impl<'a, 'r> FromRequest<'a, 'r> for Login {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Login, ()> {
        let data = request.guard::<State<DataLoad>>()?;
        let userid: String = match &request.headers().get_one("Token") {
            Some(token) => match verify_token(token) {
                Ok(userid) => userid,
                Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
            },
            None => {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        match data.inner().users.lock().unwrap().find_id(&userid) {
            Ok(user) => {
                let login = Login {
                    userid: userid,
                    name: user.get(|u| u.get_user_name().into()),
                    email: user.get(|u| u.get_user_email().into()),
                };
                Outcome::Success(login)
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
