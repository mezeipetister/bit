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
use storaget::*;

pub trait User: StorageObject {
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
pub fn get_user_by_id<'a, T>(users: &'a Storage<T>, id: &str) -> AppResult<DataObject<T>>
where
    T: User,
{
    let user = users.get_by_id(id)?;
    Ok(user)
}

/// Find user by email
/// Return NONE or &user.
pub fn get_user_by_email<'a, T>(users: &'a Storage<T>, email: &str) -> AppResult<DataObject<T>>
where
    T: User,
{
    for user in users {
        if user.get(|u| u.get_user_email() == email) {
            return Ok(user);
        }
    }
    Err(InternalError("User not found".into()))
}
