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
extern crate chrono;
extern crate core_lib;
extern crate serde_derive;

pub mod component;
pub mod guard;
pub mod layout;
pub mod login;
pub mod prelude;
pub mod view;

use core_lib::storage::StorageObject;
use core_lib::user;
use core_lib::user::User;
use core_lib::user::UserV1;
use core_lib::{storage::*, user::*};
use guard::*;
use layout::Layout;
use login::*;
use maud::Markup;
use prelude::{Check, FlashRedirect};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::Request;
use rocket::State;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use view::*;

#[get("/demo")]
fn demo(user: Login) -> Markup {
    Layout::new()
        .set_title(&format!("Hello - {}", user.name()))
        .render(ViewIndex::new().render())
}

#[get("/")]
fn index(user: Login, flash: Option<FlashMessage>) -> Result<Markup, Redirect> {
    Ok(Layout::new()
        .set_title("Welcome")
        .set_notification(flash)
        .render(ViewIndex::new().set_name(user.name()).render()))
}

#[get("/login")]
fn login(flash: Option<FlashMessage>) -> Markup {
    Layout::new()
        .set_title("Login")
        .set_notification(flash)
        .set_empty()
        .render(ViewLogin::new().render())
}

#[get("/logout")]
fn logout(_user: Login, mut cookies: Cookies) -> Result<Redirect, Redirect> {
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
fn login_post(
    mut cookies: Cookies,
    login: Form<FormLogin>,
    data: State<DataLoad>,
) -> FlashRedirect {
    // Temp login to admin
    // TODO: Remove this part, vulnerable code
    if login.username == "admin".to_owned() && login.password == "admin".to_owned() {
        return Ok(user_login(&mut cookies, "9"));
    }

    let users: &mut Vec<UserV1> = &mut data.inner().users.lock().unwrap().data;
    let user: &mut UserV1 = user::get_user_by_id(&mut *users, &login.username).check("/login")?;

    let password = &login.password;
    let hash = &user.get_password_hash();
    if password::verify_password_from_hash(&password, &hash).check("/login")? {
        return Ok(user_login(&mut cookies, &login.username));
    }

    Err(Flash::error(Redirect::to("/login"), "Failed to login."))
}

#[get("/login/reset_password")]
fn login_reset_password(flash: Option<FlashMessage>) -> Markup {
    Layout::new()
        .set_title("Reset password")
        .set_notification(flash)
        .set_empty()
        .render(ViewPasswordReset::new().render())
}

#[derive(FromForm)]
struct FormResetPassword {
    email: String,
}
#[post("/login/reset_password", data = "<form>")]
fn login_reset_password_post(
    form: Form<FormResetPassword>,
    data: State<DataLoad>,
) -> FlashRedirect {
    let _ = form.email;
    // Letd manage form.email
    let users = &mut data.inner().users.lock().unwrap().data;
    let user: &mut UserV1 = user::get_user_by_email(&mut *users, &form.email).check("/login")?;

    match &mut user.reset_password() {
        Ok(()) => {
            &user.save();
            return Ok(Redirect::to("/login/reset_password/success"));
        }
        Err(msg) => {
            println!("Error: {}", msg);
            return Ok(Redirect::to("/login/reset_password/error"));
        }
    };
}

/**
 * USERS
 */
#[get("/admin/user")]
fn admin_user(
    _user: Login,
    flash: Option<FlashMessage>,
    data: State<DataLoad>,
) -> Result<Markup, Redirect> {
    let users: &Vec<UserV1> = &data.inner().users.lock().unwrap().data;
    Ok(Layout::new()
        .set_title("Admin users")
        .set_notification(flash)
        .render(ViewAdminUser::new(users).render()))
}

#[get("/admin/user/new")]
fn admin_user_new(_user: Login, flash: Option<FlashMessage>) -> Result<Markup, Redirect> {
    Ok(Layout::new()
        .set_title("New user")
        .set_notification(flash)
        .render(ViewAdminUserNew::new().render()))
}

#[get("/settings")]
fn settings(
    user: Login,
    flash: Option<FlashMessage>,
    data: State<DataLoad>,
) -> Result<Markup, Redirect> {
    let users: &mut Vec<UserV1> = &mut data.inner().users.lock().unwrap().data;
    // TODO: Fix this. Do not use unwrap()
    let user = get_user_by_id(&mut *users, &user.userid()).unwrap();

    Ok(Layout::new()
        .set_title("Settings")
        .set_notification(flash)
        .render(ViewSettings::new(user).render()))
}

#[derive(FromForm)]
struct FormSettings {
    email: String,
    name: String,
}

#[post("/settings", data = "<form>")]
fn settings_save(user: Login, data: State<DataLoad>, form: Form<FormSettings>) -> FlashRedirect {
    let users = &mut data.inner().users.lock().unwrap().data;
    let user: &mut UserV1 = user::get_user_by_id(&mut *users, &user.userid()).check("/settings")?;
    user.update(|user| {
        user.set_user_name(form.name.clone())?;
        user.set_user_email(form.email.clone())?;
        Ok(())
    })
    .check("/settings")?;
    Ok(Redirect::to("/settings"))
}

#[get("/settings/new_password")]
fn new_password(_user: Login, flash: Option<FlashMessage>) -> Result<Markup, Redirect> {
    Ok(Layout::new()
        .set_title("New password")
        .set_notification(flash)
        .render(ViewNewPassword::new().render()))
}

#[derive(FromForm)]
struct FormNewPassword {
    password1: String,
    password2: String,
}

#[post("/settings/new_password", data = "<form>")]
fn new_password_save(
    user: Login,
    data: State<DataLoad>,
    form: Form<FormNewPassword>,
) -> FlashRedirect {
    if form.password1 != form.password2 {
        return Err(Flash::error(
            Redirect::to("/settings/new_password"),
            "Passwords are not the same!",
        ));
    }
    let users = &mut data.inner().users.lock().unwrap().data;
    let user: &mut UserV1 =
        user::get_user_by_id(&mut *users, &user.userid()).check("/settings/new_password")?;
    user.set_password(form.password1.clone())
        .check("/settings/new_password")?;
    user.save().check("/settings/new_password")?;

    Ok(Redirect::to("/settings"))
}

#[derive(FromForm)]
struct FormUserNew {
    id: String,
    name: String,
    email: String,
}

#[post("/admin/user/new", data = "<form>")]
fn admin_user_new_post(
    _user: Login,
    form: Form<FormUserNew>,
    data: State<DataLoad>,
) -> FlashRedirect {
    let new_user = UserV1::new(form.id.clone(), form.name.clone(), form.email.clone());
    let mut user_storage = data.inner().users.lock().unwrap();

    let u1 = add_to_storage_and_return_ref(&mut user_storage, new_user).check("/admin/user/new")?;
    u1.save().check("/admin/user/new")?;

    Ok(Redirect::to("/admin/user"))
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Markup {
    // println!("{:?}", req.headers());
    Layout::new()
        .set_title("404 not found")
        .set_empty()
        .render(View404::new(req.uri().path()).render())
}

#[catch(401)]
fn unauthorized(req: &Request<'_>) -> Flash<Redirect> {
    Flash::new(
        Redirect::to("/login"),
        "LOGIN_REDIRECT_TO",
        req.route().unwrap().uri.path(),
    )
}

struct DataLoad {
    users: Mutex<Storage<UserV1>>,
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
                logout,
                admin_user,
                admin_user_new,
                admin_user_new_post,
                settings,
                settings_save,
                new_password,
                new_password_save,
                login_reset_password,
                login_reset_password_post,
            ],
        )
        .register(catchers![not_found, unauthorized])
}

fn main() {
    let user_storage = load_storage::<UserV1>("data/users").unwrap();
    let data = DataLoad {
        users: Mutex::new(user_storage),
    };
    rocket(data).launch();
}
