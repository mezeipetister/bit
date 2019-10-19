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

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate core_lib;
extern crate serde_derive;

mod login;

use self::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use core_lib::{storage, user};
use login::*;
use rocket::http::{Cookies, RawStr};
use rocket::request::Form;
use rocket::response::{status, NamedFile, Redirect};
use rocket::Request;
use rocket::{Data, State};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    //name: Option<String>,
    //items: &'a Vec<T>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index(mut cookies: Cookies) -> Result<Template, Redirect> {
    if !user_auth(&mut cookies) {
        return Err(Redirect::to("/login"));
    }
    Ok(Template::render(
        "index",
        &TemplateContext {
            title: "Welcome",
            parent: "layout",
        },
    ))
}

#[get("/login")]
fn login() -> Template {
    Template::render(
        "login",
        &TemplateContext {
            title: "Login",
            parent: "layout_empty",
        },
    )
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    if !user_auth(&mut cookies) {
        return Redirect::to("/login");
    }
    user_logout(&mut cookies);
    Redirect::to("/login")
}

#[derive(FromForm)]
struct FormLogin {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
fn login_post(mut cookies: Cookies, login: Form<FormLogin>) -> Redirect {
    if login.username == "admin".to_owned() && login.password == "admin".to_owned() {
        user_login(&mut cookies, "9");
        return Redirect::to("/");
    }
    Redirect::to("/login/error")
}

#[get("/login/error")]
fn login_error() -> Template {
    Template::render(
        "login_error",
        &TemplateContext {
            title: "Login failed",
            parent: "layout_empty",
        },
    )
}

#[get("/login/reset_password")]
fn login_reset_password() -> Template {
    Template::render(
        "login_reset_password",
        &TemplateContext {
            title: "Reset password",
            parent: "layout_empty",
        },
    )
}

#[get("/login/reset_password/success")]
fn login_reset_password_success() -> Template {
    Template::render(
        "login_reset_password_success",
        &TemplateContext {
            title: "Success",
            parent: "layout_empty",
        },
    )
}

#[get("/login/reset_password/error")]
fn login_reset_password_error() -> Template {
    Template::render(
        "login_reset_password_error",
        &TemplateContext {
            title: "Error",
            parent: "layout_empty",
        },
    )
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_file,
                index,
                login,
                login_post,
                login_error,
                logout,
                login_reset_password,
                login_reset_password_success,
                login_reset_password_error
            ],
        )
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
