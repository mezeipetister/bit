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

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate core_lib;
extern crate crypto;
extern crate ifeq;
extern crate jwt;
extern crate num_format;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_derive;
extern crate storaget;

pub mod controller;
pub mod cors;
pub mod guard;
pub mod login;
pub mod model;
pub mod prelude;

use crate::prelude::*;
use core_lib::model::*;
use guard::*;
use rocket::Request;
use rocket_cors::AllowedHeaders;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use storaget::*;

#[get("/")]
fn index() -> String {
    "BIT Welcome".to_owned()
}

#[derive(Debug, Serialize)]
struct ApiWelcomeSchema {
    message: &'static str,
}

#[get("/")]
fn api_welcome(_user: Login) -> StatusOk<ApiWelcomeSchema> {
    StatusOk(ApiWelcomeSchema {
        message: "Welcome to Gardenova API",
    })
}

#[catch(404)]
fn not_found(_: &Request<'_>) -> ApiError {
    ApiError::NotFound
}

#[catch(401)]
fn unauthorized(_: &Request<'_>) -> ApiError {
    ApiError::Unauthorized
}

#[catch(422)]
fn form_error(_: &Request<'_>) -> ApiError {
    ApiError::InternalError("Minden mező kitöltése kötelező!".to_owned())
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    let mut methods = std::collections::HashSet::new();
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Post));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Get));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Put));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Delete));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Options));

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::All,
        allowed_methods: methods,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .attach(cors)
        // .attach(CORS())
        .manage(data)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                controller::login::post,
                controller::login::reset_password,
                controller::profile::profile_get,
                controller::profile::profile_post,
                controller::profile::password_change,
                controller::user::user_all_get,
                controller::user::user_id_get,
                controller::user::user_new_post,
                controller::repository::repository_all_get,
                controller::repository::repository_new_put,
                controller::repository::repository_remove_post,
                controller::repository::repository_restore_post,
                controller::repository::repository_update_post,
                controller::repository::repository_id_get,
                controller::account::account_all_get,
                controller::account::account_new_put,
                controller::account::account_id_get,
                controller::account::account_update_post,
                controller::transaction::transaction_all_get,
                controller::transaction::transaction_new_put,
                controller::transaction::transaction_id_get,
                controller::ledger::ledger_get,
                controller::ledger::ledger_stat_get,
                controller::asset::asset_all_get,
                controller::asset::asset_new_put,
                controller::asset::asset_id_get,
                controller::asset::asset_update_post,
                controller::asset::asset_remove_post,
                controller::asset::asset_restore_post,
                controller::asset::asset_statistics_by_clearing_get,
                controller::asset::asset_depreciation_yearly_get,
                controller::asset::asset_depreciation_monthly_get,
                controller::project::project_new_put,
                controller::project::project_all_get,
                controller::project::project_id_get,
                controller::project::project_update_post,
                controller::project::project_remove_post,
                controller::project::project_enable_post,
                controller::project::project_disable_post,
                controller::project::project_transaction_new_put,
                controller::project::project_transaction_remove_post,
                controller::project::project_ledger_stat_get
            ],
        )
        .register(catchers![not_found, unauthorized, form_error])
}

pub struct DataLoad {
    users: Mutex<VecPack<User>>,
    repositories: Mutex<VecPack<Repository>>,
}

fn main() -> PackResult<()> {
    let data = DataLoad {
        users: Mutex::new(VecPack::load_or_init(PathBuf::from("data/users"))?),
        repositories: Mutex::new(VecPack::load_or_init(PathBuf::from("data/repositories"))?),
    };
    rocket(data).launch();
    Ok(())
}
