// Copyright (C) 2020 Peter Mezei
//
// This file is part of BIT.
//
// BIT is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// BIT is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with BIT.  If not, see <http://www.gnu.org/licenses/>.

use crate::guard::Login;
use crate::model as ApiSchema;
use crate::prelude::*;
use crate::DataLoad;
use core_lib::model::*;
use rocket::{request::Form, State};
use rocket_contrib::json::Json;

#[get("/repository/<repository_id>/project/all")]
pub fn project_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<ApiSchema::Project>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_projects()
        .iter()
        .map(|a| (*a).clone().into())
        .collect::<Vec<ApiSchema::Project>>();
    Ok(StatusOk(res))
}

#[put("/repository/<repository_id>/project/new", data = "<form>")]
pub fn project_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<ApiSchema::ProjectNew>,
    repository_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let project_new = Project::new(
        form.name.clone(),
        form.description.clone(),
        user.userid().to_string(),
    );
    let p = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .add_project(
            project_new.name,
            project_new.description,
            user.userid().to_string(),
        )?
        .clone();
    Ok(StatusOk(p.into()))
}

#[get("/repository/<repository_id>/project/<project_id>", rank = 2)]
pub fn project_id_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_project_by_id(&project_id)?
        .clone();
    Ok(StatusOk(res.into()))
}

#[post(
    "/repository/<repository_id>/project/<project_id>",
    data = "<form>",
    rank = 3
)]
pub fn project_update_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
    form: Json<ApiSchema::ProjectUpdate>,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .update_project(
            &project_id,
            form.name.clone(),
            form.description.clone(),
            form.is_enabled,
        )?
        .clone();
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/remove", rank = 3)]
pub fn project_remove_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .remove_project_by_id(&project_id)?;
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/enable", rank = 3)]
pub fn project_enable_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .enable_project(&project_id)?
        .clone();
    Ok(StatusOk(res.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/disable", rank = 3)]
pub fn project_disable_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Project>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .disable_project(&project_id)?
        .clone();
    Ok(StatusOk(res.into()))
}

#[put(
    "/repository/<repository_id>/project/<project_id>/transaction/new",
    data = "<form>"
)]
pub fn project_transaction_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<ApiSchema::TransactionNew>,
    repository_id: String,
    project_id: String,
) -> Result<StatusOk<ApiSchema::Transaction>, ApiError> {
    let p = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .add_project_transaction(
            &project_id,
            form.subject.clone(),
            form.debit.clone(),
            form.credit.clone(),
            form.amount,
            form.date_settlement,
            user.userid().to_string(),
        )?
        .clone();
    Ok(StatusOk(p.into()))
}

#[post("/repository/<repository_id>/project/<project_id>/transaction/<transaction_id>/remove")]
pub fn project_transaction_remove_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    project_id: String,
    transaction_id: usize,
) -> Result<StatusOk<Vec<ApiSchema::Transaction>>, ApiError> {
    let p = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .remove_project_transaction_by_id(&project_id, transaction_id)?
        .clone();
    Ok(StatusOk(
        p.iter()
            .map(|t| t.clone().into())
            .collect::<Vec<ApiSchema::Transaction>>(),
    ))
}

#[derive(FromForm)]
pub struct StatFilter {
    account: String,
}

#[get("/repository/<repository_id>/project/stat?<filter..>")]
pub fn project_ledger_stat_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    filter: Form<StatFilter>,
) -> Result<StatusOk<[Option<i32>; 12]>, ApiError> {
    // Init result array
    let mut result: [Option<i32>; 12] = [None; 12];

    let mut transactions: Vec<Transaction> = Vec::new();
    for project in data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_projects_enabled()
    {
        for transaction in project.get_transactions().iter() {
            transactions.push(transaction.clone());
        }
    }
    for transaction in transactions {
        if transaction.date_settlement.year() != Utc::now().naive_utc().year() {
            continue;
        }
        let month = (transaction.date_settlement.month() - 1) as usize;
        if transaction.debit.starts_with(&filter.account) {
            match result[month] {
                Some(value) => result[month] = Some(value + transaction.amount),
                None => result[month] = Some(transaction.amount),
            }
        }
        if transaction.credit.starts_with(&filter.account) {
            match result[month] {
                Some(value) => result[month] = Some(value - transaction.amount),
                None => result[month] = Some(transaction.amount * -1),
            }
        }
    }
    Ok(StatusOk(result))
}
