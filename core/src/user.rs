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

use crate::error::Error::*;
use crate::model::User;
use crate::prelude::*;
use storaget::*;

// pub trait User: StorageObject {
//     fn get_user_id(&self) -> &str;
//     fn set_user_id(&mut self, user_id: String) -> AppResult<()>;
//     fn get_user_name(&self) -> &str;
//     fn set_user_name(&mut self, name: String) -> AppResult<()>;
//     fn get_user_email(&self) -> &str;
//     fn set_user_email(&mut self, email: String) -> AppResult<()>;
//     fn get_user_phone(&self) -> &str;
//     fn set_user_phone(&mut self, phone: String) -> AppResult<()>;
//     fn get_password_hash(&self) -> &str;
//     fn set_password(&mut self, password: String) -> AppResult<()>;
//     fn reset_password(&mut self) -> AppResult<()>;
// }

/// Find user in users by ID.
/// Return NONE if not exist, return &user if exists.
pub fn get_user_by_id<'a>(users: &'a mut VecPack<User>, id: &str) -> AppResult<&'a mut Pack<User>> {
    let user = users.find_id_mut(id)?;
    Ok(user)
}

/// Find user by email
/// Return NONE or &user.
pub fn get_user_by_email<'a>(
    users: &'a mut Vec<Pack<User>>,
    email: &str,
) -> AppResult<&'a mut Pack<User>> {
    for user in users {
        if user.get_user_email() == email {
            return Ok(user);
        }
    }
    Err(InternalError("User not found".into()))
}
