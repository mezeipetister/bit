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

use core_lib::error::Error;
use core_lib::prelude::AppResult;
use crypto::sha2::Sha256;
use jwt::{Header, Token};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Default, Deserialize, Serialize, RustcDecodable, RustcEncodable)]
struct Custom {
    uid: String,
    rhino: bool,
}

pub fn create_token(user_id: &str) -> AppResult<String> {
    let header: Header = Default::default();
    let claims = Custom {
        uid: user_id.into(),
        rhino: true,
        ..Default::default()
    };
    let token = Token::new(header, claims);

    match token.signed(b"secret_key", Sha256::new()) {
        Ok(token) => return Ok(token),
        Err(_) => {
            return Err(Error::InternalError(
                "Hiba a JWT Token készítésekor, login failed".to_owned(),
            ))
        }
    }
}

pub fn verify_token(token: &str) -> AppResult<String> {
    let token = match Token::<Header, Custom>::parse(token) {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::InternalError(
                "Hibás authentikációs TOKEN!".to_owned(),
            ))
        }
    };

    if token.verify(b"secret_key", Sha256::new()) {
        Ok(token.claims.uid)
    } else {
        Err(Error::InternalError(
            "Hibás authentikációs TOKEN!".to_owned(),
        ))
    }
}
