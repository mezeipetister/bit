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

use crate::email;
use crate::email::*;
use crate::error::Error::*;
use crate::prelude::*;
use crate::storage;
use crate::user::password::*;
use crate::user::User;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserV1 {
    id: String,
    path: Option<String>,
    name: String,
    email: String,
    phone: String,
    password_hash: String,
    date_added: DateTime<Utc>,
}

impl UserV1 {
    pub fn new(id: String, name: String, email: String) -> Self {
        UserV1 {
            id,
            path: None,
            name,
            email,
            phone: "".into(),
            password_hash: "".into(),
            date_added: Utc::now(),
        }
    }
}

impl User for UserV1 {
    fn get_user_id(&self) -> &str {
        &self.id
    }
    fn set_user_id(&mut self, user_id: String) -> AppResult<()> {
        if user_id.len() <= 5 {
            Err(InternalError(
                "User ID must has lenght min 5 characters.".into(),
            ))
        } else {
            // Here we set ID as all lowecase
            self.id = user_id.to_lowercase();
            Ok(())
        }
    }
    fn get_user_name(&self) -> &str {
        &self.name
    }
    fn set_user_name(&mut self, name: String) -> AppResult<()> {
        if name.len() < 5 {
            Err(InternalError(
                "User name must be longer then 5 character".into(),
            ))
        } else {
            self.name = name.to_string();
            Ok(())
        }
    }
    fn get_user_email(&self) -> &str {
        &self.email
    }
    fn set_user_email(&mut self, email: String) -> AppResult<()> {
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            self.email = email;
            Ok(())
        } else {
            Err(InternalError(
                "Wrong email format! Email must contains the followings: 
                @ and . . Len must be higher then 5 characters"
                    .into(),
            ))
        }
    }
    fn get_user_phone(&self) -> &str {
        &self.phone
    }
    fn set_user_phone(&mut self, phone: String) -> AppResult<()> {
        if phone.len() > 5 {
            self.phone = phone;
            Ok(())
        } else {
            Err(InternalError(
                "Phone number must be higher then 5 characters. It seems to be wrong format."
                    .into(),
            ))
        }
    }
    fn get_password_hash(&self) -> &str {
        &self.password_hash
    }
    fn set_password(&mut self, password: String) -> AppResult<()> {
        validate_password(&password)?;
        self.password_hash = hash_password(&password)?;
        Ok(())
    }

    // TODO: Maybe should be at a higher level using User trait reference as input?
    // Maybe this?
    // => fn reset_password<T: User>(user: &T) -> Result<(), String> {...}
    fn reset_password(&mut self) -> AppResult<()> {
        let new_password = generate_random_password(None)?;
        self.password_hash = hash_password(&new_password)?;
        match email::new(
            &self.get_user_email(),
            "New password generated",
            &format!(
                "Your username is: {}\nYour new password is: {}",
                self.get_user_id(),
                &new_password
            ),
        )
        .send()
        {
            Ok(_) => (),
            // TODO:
            // Use email pool, in case of email service failure.
            // Instead of using error in case of error - directly here -,
            // We should say its Ok(()) now, and in case of error, the email pool,
            // should manage the trials.
            Err(msg) => {
                return Err(InternalError(format!(
                    "New password generated and set, but email send faild. Error message: {}",
                    msg
                )))
            }
        }
        Ok(())
    }
}

/**
 * StorageObject implementation for UserObject
 */
impl storage::StorageObject for UserV1 {
    fn get_id(&self) -> &str {
        &self.id
    }
    fn save(&self) -> AppResult<()> {
        storage::save_storage_object(self)
    }
    // TODO: Fix this one!
    fn reload(&mut self) -> AppResult<()> {
        Ok(())
    }
    fn get_path(&self) -> Option<&str> {
        match &self.path {
            Some(path) => Some(path.as_ref()),
            None => None,
        }
    }
    fn set_path(&mut self, path: &str) -> AppResult<()> {
        self.path = Some(path.into());
        Ok(())
    }
    fn get_date_created(&self) -> DateTime<Utc> {
        self.date_added
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());
        // At this point ID should be None;
        assert_eq!(user.get_user_id(), "demo");
        // This should return an Err(..)
        // Let's test is
        assert_eq!(user.set_user_id("de".into()).is_err(), true);
        // Now the user should have Some("demo_user" as String) as ID.
        // Test that it's not overwritten, and all letter is lovercase
        assert_eq!(user.get_user_id(), "demo");
    }

    #[test]
    fn test_user_email() {
        let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());

        assert_eq!(user.set_user_email("demo@demo.com".into()).is_ok(), true); // should be ok
        assert_eq!(user.set_user_email("wohoo".into()).is_err(), true); // should be err
        assert_eq!(user.set_user_email("demo@company.com".into()).is_ok(), true); // should be ok

        // Check email wether email is correct
        assert_eq!(user.get_user_email(), "demo@company.com");
    }

    #[test]
    fn test_user_name() {
        let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());
        assert_eq!(user.get_user_name(), "user");
        assert_eq!(user.set_user_name("abc".into()).is_err(), true); // should be err
        assert_eq!(user.set_user_name("Demo User".into()).is_ok(), true); // should be ok
        assert_eq!(user.set_user_name("Hello World".into()).is_ok(), true); // should be ok
        assert_eq!(user.get_user_name(), "Hello World"); // should be ok
    }

    #[test]
    fn test_user_phone() {
        let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());
        let phone_number: &str = "+99 (701) 479 397129";
        assert_eq!(user.get_user_phone(), "");
        assert_eq!(user.set_user_phone(phone_number.into()).is_ok(), true); // should be ok
        assert_eq!(user.set_user_phone("phn".into()).is_err(), true); // should be err
        assert_eq!(user.get_user_phone(), phone_number);
    }

    #[test]
    fn test_user_set_password() {
        let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());
        let password: &str = "HelloWorld749";
        assert_eq!(user.get_password_hash(), ""); // should be None
        assert_eq!(user.set_password("pass".into()).is_ok(), false); // should be err
        assert_eq!(user.set_password("PAss7".into()).is_ok(), true); // should be err
        assert_eq!(user.set_password("password".into()).is_ok(), false); // should be err
        assert_eq!(user.set_password("Password".into()).is_ok(), false); // should be err
        assert_eq!(user.set_password("PAssword".into()).is_ok(), false); // should be err
        assert_eq!(user.set_password("PAssword7".into()).is_ok(), true); // should be ok
        assert_eq!(user.set_password(password.into()).is_ok(), true); // should be ok
        assert_eq!(
            verify_password_from_hash(password, user.get_password_hash()).unwrap(),
            true
        );
    }
    // #[test]
    // #[ignore]
    // fn test_reset_password() {
    //     let mut user: UserV1 = UserV1::new("demo".into(), "user".into(), "demo@user.com".into());
    //     user.set_user_email(&env::var("E_TO_TEST_EMAIL").unwrap())
    //         .unwrap();
    //     user.set_user_name(&env::var("E_TO_TEST_NAME").unwrap())
    //         .unwrap();
    //     assert_eq!(user.reset_password().is_ok(), true);
    // }
}
