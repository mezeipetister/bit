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

pub mod login;
pub mod model;
pub mod password;

pub use model::UserV1;

use crate::error::Error::*;
use crate::prelude::*;

pub trait User {
    fn get_user_id(&self) -> &str;
    fn set_user_id(&mut self, user_id: String) -> AppResult<()>;
    fn get_user_name(&self) -> &str;
    fn set_user_name(&mut self, name: String) -> AppResult<()>;
    fn get_user_email(&self) -> &str;
    fn set_user_email(&mut self, email: String) -> AppResult<()>;
    fn get_user_phone(&self) -> &str;
    fn set_user_phone(&mut self, phone: String) -> AppResult<()>;
    fn get_password_hash(&self) -> &str;
    fn set_password(&mut self, password: String) -> AppResult<()>;
    fn reset_password(&mut self) -> AppResult<()>;
}

/// Find user in users by ID.
/// Return NONE if not exist, return &user if exists.
pub fn get_user_by_id<'a, T: User>(users: &'a mut Vec<T>, id: &str) -> AppResult<&'a mut T> {
    for user in users {
        if user.get_user_id() == id {
            return Ok(&mut *user);
        }
    }
    Err(InternalError("User not found".into()))
}

/// Find user by email
/// Return NONE or &user.
pub fn get_user_by_email<'a, T: User>(users: &'a mut Vec<T>, email: &str) -> AppResult<&'a mut T> {
    for user in users {
        if user.get_user_email() == email {
            return Ok(&mut *user);
        }
    }
    Err(InternalError("User not found".into()))
}
