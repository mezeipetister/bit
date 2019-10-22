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

use maud::Markup;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::Route;

pub fn user_auth(mut cookies: &mut Cookies, route: &Route) -> Result<(), Redirect> {
    if cookie_get_private(&mut cookies, "USERID").is_none() {
        cookies.add(Cookie::new("REDIRECT", format!("{}", route.uri.path())));
        return Err(Redirect::to("/login"));
    }
    Ok(())
}

pub fn user_login(cookies: &mut Cookies, userid: &'static str) -> Redirect {
    cookie_set_private(cookies, "USERID", userid);
    if let Some(redirect_to) = cookies.get("REDIRECT") {
        return Redirect::to(redirect_to.value().to_owned());
    }
    Redirect::to("/")
}

pub fn user_logout(cookies: &mut Cookies) {
    cookies.remove_private(Cookie::named("USERID"));
}

pub fn cookie_set_message(cookies: &mut Cookies, message: &'static str) {
    cookies.add_private(Cookie::new("message", message));
}

pub fn cookie_get_message(cookies: &mut Cookies) -> Option<String> {
    match cookies.get_private("message") {
        Some(message) => {
            let res = message;
            cookies.remove_private(Cookie::named("message"));
            return Some(res.value().to_owned());
        }
        None => return None,
    }
}

pub fn cookie_set(cookies: &mut Cookies, key: &'static str, value: &'static str) {
    cookies.add(Cookie::new(key, value));
}

pub fn cookie_set_private(cookies: &mut Cookies, key: &'static str, value: &'static str) {
    cookies.add_private(Cookie::new(key, value));
}

pub fn cookie_get(cookies: &mut Cookies, key: &'static str) -> Option<String> {
    match cookies.get(key) {
        Some(value) => {
            let res = value;
            return Some(res.value().to_owned());
        }
        None => return None,
    }
}

pub fn cookie_get_private(cookies: &mut Cookies, key: &'static str) -> Option<String> {
    match cookies.get_private(key) {
        Some(value) => {
            let res = value;
            return Some(res.value().to_owned());
        }
        None => return None,
    }
}
