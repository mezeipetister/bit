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

use core_lib::Error;
use maud::Markup;
use rocket::response::{Flash, Redirect};
use std::fmt::Display;

pub type FlashRedirect = Result<Redirect, Flash<Redirect>>;
pub type FlashOk = Result<Markup, Flash<Redirect>>;

pub trait Check<T> {
    fn check(self, redirect_to: &str) -> Result<T, Flash<Redirect>>;
}

pub trait CheckError<T> {
    fn check_error(self, err: Error, redirect_to: &str) -> Result<T, Flash<Redirect>>;
}

impl<T, E> Check<T> for Result<T, E>
where
    E: Display,
{
    fn check(self, redirect_to: &str) -> Result<T, Flash<Redirect>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(msg) => Err(Flash::warning(
                Redirect::to(redirect_to.to_owned()),
                format!("{}", msg),
            )),
        }
    }
}

impl<T, E> CheckError<T> for Result<T, E>
where
    E: Display,
{
    fn check_error(self, err: Error, redirect_to: &str) -> Result<T, Flash<Redirect>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Flash::warning(
                Redirect::to(redirect_to.to_owned()),
                format!("{}", err),
            )),
        }
    }
}
