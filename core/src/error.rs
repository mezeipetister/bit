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
#[rustfmt::skip]
pub enum ErrorCode {
    // Page not found
    E404NotFound                = 404,
    // Internal Error
    E500InternalError           = 500,
    E700WrongEmailFormat        = 700,
    E701WeakPassword            = 701,
    E702PermissionDenied        = 702,
    E703TimeOut                 = 703,
    E704OperationTimeOut        = 704,
    // 705 Request permission
    // denied
    E705RequestPermissionDenied = 705,
    // 706 - Permission Denied
    E706IOPermissionDenied      = 706,
    // # CrateError
    // 
    // General UNKNOWN error for crate.
    // Use it when we have no clue what is
    // going on.
    E900CrateError              = 900,
}

// Implementing Debug trait for ErrorCode
// It's needed for #[derive(Debug)] AppError
impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

pub enum ErrorKind {
    // Error in our crate(s)
    // It holds a static str reference
    // with the crate name.
    // e.g.: ErrorKind::CrateError("core")
    CrateError(&'static str),
    // Error in a third party package we use
    // It holds a static str reference
    // with the package name.
    // e.g.: ErrorKind::PackageError("email")
    PackageError(&'static str),
}

/// Error Trait for core errors.
pub trait Error {
    // Optionally set error code
    fn set_error_code(&mut self, code: &str) -> &mut Self;
    /// Optionally set error kind
    fn set_kind(&mut self, kind: &str) -> &mut Self;
}

#[derive(Debug)]
pub struct AppError {
    code: Option<ErrorCode>,
    kind: Option<String>,
    message: Option<String>,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            code: None,
            kind: None,
            message: None,
        }
    }
}

// Well formatted display text for users
// TODO: Use error code and language translation for end-user error messages.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.message {
            Some(message) => write!(f, "{}", message),
            None => write!(f, "Error, but no error message"),
        }
    }
}

impl From<&'static str> for AppError {
    fn from(error: &'static str) -> Self {
        AppError::new(error)
    }
}
