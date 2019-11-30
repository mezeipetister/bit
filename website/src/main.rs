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
extern crate storaget;

pub mod component;
pub mod guard;
pub mod layout;
pub mod login;
pub mod prelude;
pub mod view;

use crate::core_lib::{Account, Transaction};
use crate::prelude::CheckError;
use crate::prelude::FlashOk;
use chrono::prelude::*;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::User;
use core_lib::user::UserV1;
use core_lib::user::*;
use core_lib::Account1;
use core_lib::Transaction1;
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
use std::str::FromStr;
use storaget::*;
use view::*;

#[get("/demo")]
fn demo(user: Login) -> Markup {
    Layout::new()
        .set_title(&format!("Hello - {}", user.name()))
        .render(ViewIndex::new().set_name("Wohoo").render())
}

#[get("/")]
fn index(user: Login, flash: Option<FlashMessage>) -> Result<Markup, Redirect> {
    Ok(Layout::new()
        .set_title("Welcome")
        .set_notification(flash)
        .render(
            ViewIndex::new()
                .set_name(&format!("{}", user.name()))
                .render(),
        ))
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

    let user = &data.inner().users.get_by_id(&login.username).check_error(
        core_lib::Error::InternalError("User not found".to_owned()),
        "/login",
    )?;

    let password = &login.password;
    let hash = user.get(|u| u.get_password_hash().to_owned());
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
    let user = user::get_user_by_email(&data.inner().users, &form.email).check("/login")?;

    match &user.update(|u| u.reset_password()) {
        Ok(()) => {
            return Ok(Redirect::to("/login/reset_password/success"));
        }
        Err(msg) => {
            println!("Error: {}", msg);
            return Ok(Redirect::to("/login/reset_password/error"));
        }
    };
}

// Accounts
#[get("/accounts")]
fn accounts_get(flash: Option<FlashMessage>, data: State<DataLoad>) -> Markup {
    Layout::new()
        .set_title("Accounts")
        .set_notification(flash)
        .render(ViewAccount::new(&data.inner().accounts, &data.inner().transactions).render())
}

// Accounts
#[get("/accounts/<account_id>")]
fn accounts_edit_get(
    flash: Option<FlashMessage>,
    data: State<DataLoad>,
    account_id: String,
) -> FlashOk {
    Ok(Layout::new()
        .set_title("Accounts")
        .set_notification(flash)
        .render(
            ViewAccountEdit::new(
                data.inner()
                    .accounts
                    .get_by_id(&account_id)
                    .check("/accounts")?,
            )
            .render(),
        ))
}

#[derive(FromForm)]
struct FormAccountEdit {
    account_name: String,
    account_description: String,
    is_working: bool,
    is_inverse: bool,
}
#[post("/accounts/<account_id>", data = "<form>")]
fn accounts_edit_post(
    _user: Login,
    form: Form<FormAccountEdit>,
    data: State<DataLoad>,
    account_id: String,
) -> FlashRedirect {
    let account_id = account_id.trim();
    data.inner()
        .accounts
        .get_by_id(&account_id)
        .check(&format!("/accounts/{}", account_id))?
        .update(|a| {
            (*a).set_description(&form.account_description)?;
            (*a).set_name(&form.account_name)?;
            (*a).set_inverse(form.is_inverse)?;
            (*a).set_working(form.is_working)
        })
        .check(&format!("/accounts/{}", account_id))?;
    Ok(Redirect::to(format!("/accounts/{}", account_id)))
}

// Accounts new
#[get("/accounts/new")]
fn accounts_new_get(flash: Option<FlashMessage>) -> Markup {
    Layout::new()
        .set_title("Accounts")
        .set_notification(flash)
        .render(ViewAccountNew::new().render())
}

#[derive(FromForm)]
struct FormAccount {
    account_id: String,
    account_name: String,
    account_description: String,
    is_working: bool,
    is_inverse: bool,
}
#[post("/accounts/new", data = "<form>")]
fn accounts_new_post(user: Login, form: Form<FormAccount>, data: State<DataLoad>) -> FlashRedirect {
    let new_account = Account1::new(&form.account_id, &user.userid()).check("/accounts/new")?;
    data.inner()
        .accounts
        .add_to_storage(new_account)
        .check("/accounts/new")?;
    data.inner()
        .accounts
        .get_by_id(&form.account_id)
        .unwrap()
        .update(|a| {
            (*a).set_description(&form.account_description)?;
            (*a).set_name(&form.account_name)?;
            (*a).set_inverse(form.is_inverse)?;
            (*a).set_working(form.is_working)
        })
        .check("/accounts/new")?;
    Ok(Redirect::to("/accounts"))
}

// Transaction new
#[get("/transactions/new")]
fn transaction_new_get(flash: Option<FlashMessage>) -> Markup {
    Layout::new()
        .set_title("New transaction")
        .set_notification(flash)
        .render(ViewTransactionNew::new().render())
}

#[derive(FromForm)]
struct FormTransaction {
    transaction_subject: String,
    transaction_debit: String,
    transaction_credit: String,
    transaction_amount: u32,
    transaction_date_settlement: String,
}
// Transaction new post
#[post("/transactions/new", data = "<form>")]
fn transaction_new_post(
    user: Login,
    form: Form<FormTransaction>,
    data: State<DataLoad>,
) -> FlashRedirect {
    let new_transaction = Transaction1::new(
        form.transaction_subject.clone(),
        form.transaction_debit.clone(),
        form.transaction_credit.clone(),
        form.transaction_amount,
        NaiveDate::from_str(&form.transaction_date_settlement).check_error(
            core_lib::error::Error::InternalError("Wrong date parse".to_owned()),
            "/transactions/new",
        )?,
        user.userid().to_owned(),
        &data.inner().accounts,
    )
    .check("/transactions/new")?;
    data.inner()
        .transactions
        .add_to_storage(new_transaction)
        .check("/transactions/new")?;
    Ok(Redirect::to("/transactions"))
}

// Transaction new
#[get("/transactions")]
fn transaction_get(flash: Option<FlashMessage>, data: State<DataLoad>) -> Markup {
    println!("{:?}", &data.inner().transactions.data());
    Layout::new()
        .set_title("New transaction")
        .set_notification(flash)
        .render(ViewTransaction::new(&data.inner().transactions).render())
}

// Dashboard
#[get("/dashboard")]
fn dashboard_get(flash: Option<FlashMessage>, data: State<DataLoad>) -> Markup {
    Layout::new()
        .set_title("Dashboard")
        .set_notification(flash)
        .render(ViewDashboard::new(&data.inner().transactions).render())
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
    Ok(Layout::new()
        .set_title("Admin users")
        .set_notification(flash)
        .render(ViewAdminUser::new(&data.inner().users).render()))
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
    // TODO: Fix this. Do not use unwrap()
    let user = get_user_by_id(&data.inner().users, &user.userid()).unwrap();

    Ok(Layout::new()
        .set_title("Settings")
        .set_notification(flash)
        .disable_tabbar()
        .render(ViewSettings::new(&user.clone_data()).render()))
}

#[derive(FromForm)]
struct FormSettings {
    email: String,
    name: String,
}

#[post("/settings", data = "<form>")]
fn settings_save(user: Login, data: State<DataLoad>, form: Form<FormSettings>) -> FlashRedirect {
    user::get_user_by_id(&data.inner().users, &user.userid())
        .check("/settings")?
        .update(|user| -> AppResult<()> {
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
        .disable_tabbar()
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
    user::get_user_by_id(&data.inner().users, &user.userid())
        .check("/settings/new_password")?
        .update(|u| u.set_password(form.password1.clone()))
        .check("/settings/new_password")?;

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
    let new_user = UserV1::new(form.id.clone(), form.name.clone(), form.email.clone())
        .check("/admin/user/new")?;
    data.inner().users.add_to_storage(new_user).unwrap();

    // let u1 = add_to_storage_and_return_ref(&mut user_storage, new_user).check("/admin/user/new")?;
    // u1.save().check("/admin/user/new")?;

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
                accounts_get,
                accounts_edit_get,
                accounts_edit_post,
                accounts_new_get,
                accounts_new_post,
                transaction_get,
                transaction_new_get,
                transaction_new_post,
                dashboard_get,
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

struct DataLoad {
    users: Storage<UserV1>,
    accounts: Storage<Account1>,
    transactions: Storage<Transaction1>,
}

fn main() -> StorageResult<()> {
    let data = DataLoad {
        users: Storage::load_or_init::<UserV1>("data/users")?,
        accounts: Storage::load_or_init::<Account1>("data/accounts")?,
        transactions: Storage::load_or_init::<Transaction1>("data/transactions")?,
    };
    rocket(data).launch();
    Ok(())
}
