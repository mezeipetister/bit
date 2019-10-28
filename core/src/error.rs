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

use std::error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    kind: String,
    message: String,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            kind: "Crate error".into(),
            message: message.into(),
        }
    }
}

pub fn error<T>(message: &str) -> Result<T, AppError> {
    Err(AppError {
        kind: "Crate error".into(),
        message: message.into(),
    })
}

// Well formatted display text for users
// TODO: Use error code and language translation for end-user error messages.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&'static str> for AppError {
    fn from(error: &'static str) -> Self {
        AppError::new(error)
    }
}
