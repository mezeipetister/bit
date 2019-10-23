// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2; of the License, or
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

pub mod component;
pub mod layout;
pub mod login;
pub mod view;

use core_lib::{storage::*, user::*};
use layout::Layout;
use login::*;
use maud::Markup;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use rocket::Route;
use rocket::State;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use view::*;

#[get("/demo")]
fn demo(mut cookies: Cookies, route: &Route) -> Result<Markup, Redirect> {
    user_auth(&mut cookies, route)?;
    Ok(Layout::new()
        .set_title("Wohoo")
        .render(ViewIndex::new().render()))
}

#[get("/")]
fn index(mut cookies: Cookies, route: &Route) -> Result<Markup, Redirect> {
    user_auth(&mut cookies, route)?;
    Ok(Layout::new()
        .set_title("Welcome")
        .render(ViewIndex::new().render()))
}

#[get("/login")]
fn login() -> Markup {
    Layout::new()
        .set_title("Login")
        .set_empty()
        .render(ViewLogin::new().render())
}

#[get("/logout")]
fn logout(mut cookies: Cookies, route: &Route) -> Result<Redirect, Redirect> {
    // Check wheter user is logged in
    user_auth(&mut cookies, route)?;
    // Remove userid cookie
    user_logout(&mut cookies);
    // Redirect to /login page
    Ok(Redirect::to("/login"))
}

#[derive(FromForm)]
struct FormLogin {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
fn login_post(mut cookies: Cookies, login: Form<FormLogin>) -> Redirect {
    if login.username == "admin".to_owned() && login.password == "admin".to_owned() {
        return user_login(&mut cookies, "9");
    }
    Redirect::to("/login/error")
}

#[get("/login/error")]
fn login_error() -> Markup {
    Layout::new()
        .set_title("Login failed")
        .set_empty()
        .render(ViewLogin::new().render_error())
}

#[get("/login/reset_password")]
fn login_reset_password() -> Markup {
    Layout::new()
        .set_title("Reset password")
        .set_empty()
        .render(ViewPasswordReset::new().render())
}

#[derive(FromForm)]
struct FormResetPassword {
    email: String,
}
// TODO: Implement! Now its just dummy.
#[post("/login/reset_password", data = "<form>")]
fn login_reset_password_post(form: Form<FormResetPassword>) -> Redirect {
    let _ = form.email;
    // Letd manage form.email
    Redirect::to("/login/reset_password/success")
}

// TODO: Implement! Now its just dummy.
#[get("/login/reset_password/success")]
fn login_reset_password_success() -> Markup {
    Layout::new()
        .set_title("Reset password")
        .set_empty()
        .render(ViewPasswordReset::new().render_success())
}

// TODO: Implement! Now its just dummy.
#[get("/login/reset_password/error")]
fn login_reset_password_error() -> Markup {
    Layout::new()
        .set_title("Reset password")
        .set_empty()
        .render(ViewPasswordReset::new().render_error())
}

/**
 * USERS
 */
#[get("/admin/user")]
fn admin_user(
    mut cookies: Cookies,
    route: &Route,
    data: State<DataLoad>,
) -> Result<Markup, Redirect> {
    user_auth(&mut cookies, route)?;

    let users: &Vec<UserObject> = &data.inner().users.lock().unwrap().data;
    Ok(Layout::new()
        .set_title("Admin users")
        .render(ViewAdminUser::new(users).render()))
}

#[get("/admin/user/new")]
fn admin_user_new(mut cookies: Cookies, route: &Route) -> Result<Markup, Redirect> {
    user_auth(&mut cookies, route)?;
    Ok(Layout::new()
        .set_title("New user")
        .render(ViewAdminUserNew::new().render()))
}

#[derive(FromForm)]
struct FormUserNew {
    id: String,
    name: String,
    email: String,
}

#[post("/admin/user/new", data = "<form>")]
fn admin_user_new_post(
    mut cookies: Cookies,
    route: &Route,
    form: Form<FormUserNew>,
    data: State<DataLoad>,
) -> Redirect {
    match user_auth(&mut cookies, route) {
        Ok(_) => (),
        Err(redirect) => return redirect,
    };

    let mut new_user = UserObject::new();
    new_user.set_user_id(form.id.as_ref()).unwrap();
    new_user.set_user_name(form.name.as_ref()).unwrap();
    new_user.set_user_email(form.email.as_ref()).unwrap();

    let mut user_storage = data.inner().users.lock().unwrap();

    let u1 = add_to_storage_and_return_ref(&mut user_storage, new_user).unwrap();
    u1.save().unwrap();

    Redirect::to("/admin/user")
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Markup {
    Layout::new()
        .set_title("404 not found")
        .set_empty()
        .render(View404::new(req.uri().path()).render())
}

struct DataLoad {
    users: Mutex<Storage<UserObject>>,
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    rocket::ignite()
        .manage(data)
        .mount(
            "/",
            routes![
                demo,
                static_file,
                index,
                login,
                login_post,
                login_error,
                logout,
                admin_user,
                admin_user_new,
                admin_user_new_post,
                login_reset_password,
                login_reset_password_post,
                login_reset_password_success,
                login_reset_password_error
            ],
        )
        .register(catchers![not_found])
}

fn main() {
    let user_storage = load_storage::<UserObject>("data/users").unwrap();
    let data = DataLoad {
        users: Mutex::new(user_storage),
    };
    rocket(data).launch();
}
