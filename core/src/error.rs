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

use std::fmt;

/// ErrorCode table
/// Contains all the managed error codes
/// We can use it for multi language text
/// translations.
/// 
/// 5XX =>  Format error
///         Input data validation e.g.: email validation,
///         missing field, input length, weak password.
/// 
/// XXX =>  Internal error
/// 
/// 9XX =>  Package error
///         3rd party package error.
#[rustfmt::skip]
#[derive(Copy, Clone)]
pub enum ErrorCode {
    Empty               = 0,
    // Errors from 3rd party crate
    E500InternalError   = 500,
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

pub struct AppError {
    error_code: ErrorCode,
    message: Option<String>,
}

// Well formatted display text for users
// TODO: Use error code and language translation for end-user error messages.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ErrorCode: {}. Error message: {}",
            self.error_code,
            self.message.as_ref().unwrap_or(&"-".into()),
        )
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ErrorCode: {}. Error message: {}",
            self.error_code,
            self.message.as_ref().unwrap_or(&"-".into()),
        )
    }
}

/// New error with message
pub fn new(message: &str) -> AppError {
    AppError {
        error_code: ErrorCode::Empty,
        message: Some(message.into()),
    }
}

/// New error with error code + message
pub fn new_with_code(error_code: ErrorCode, message: &str) -> AppError {
    AppError {
        error_code: error_code,
        message: Some(message.into()),
    }
}

// AppError::from(&str);
impl From<&'static str> for AppError {
    fn from(error: &'static str) -> Self {
        new(error)
    }
}
