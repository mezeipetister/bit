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
use rocket::State;
use rocket_contrib::json::Json;

#[get("/repository/<repository_id>/asset/all")]
pub fn asset_all_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<ApiSchema::AssetShort>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_assets()
        .iter()
        .filter(|a| a.get_is_active())
        .map(|a| a.clone().into())
        .collect::<Vec<ApiSchema::AssetShort>>();
    Ok(StatusOk(res))
}

#[put("/repository/<repository_id>/asset/new", data = "<form>")]
pub fn asset_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<ApiSchema::AssetNew>,
    repository_id: String,
) -> Result<StatusOk<ApiSchema::Asset>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .add_asset(
            form.name.clone(),
            form.description.clone(),
            form.account.clone(),
            form.account_clearing.clone(),
            form.value,
            form.date_activated,
            form.depreciation_key,
            form.residual_value,
            user.userid().to_string(),
        )?
        .into();
    Ok(StatusOk(res))
}

#[get("/repository/<repository_id>/asset/<asset_id>", rank = 2)]
pub fn asset_id_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    asset_id: usize,
) -> Result<StatusOk<ApiSchema::Asset>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_asset_by_id(asset_id)?
        .into();
    Ok(StatusOk(res))
}

#[get("/repository/<repository_id>/asset/clearing_statistics", rank = 5)]
pub fn asset_statistics_by_clearing_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
) -> Result<StatusOk<Vec<(String, u32, u32, u32)>>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_statistics_by_account_clearings();
    Ok(StatusOk(res))
}

#[get(
    "/repository/<repository_id>/asset/depreciation_yearly/<year>",
    rank = 5
)]
pub fn asset_depreciation_yearly_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    year: i32,
) -> Result<StatusOk<u32>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_yearly_depreciation(year);
    Ok(StatusOk(res))
}

#[get(
    "/repository/<repository_id>/asset/depreciation_monthly/<year>/<month>",
    rank = 5
)]
pub fn asset_depreciation_monthly_get(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    year: i32,
    month: u32,
) -> Result<StatusOk<u32>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id(&repository_id)?
        .get_monthly_depreciation(year, month);
    Ok(StatusOk(res))
}

#[post(
    "/repository/<repository_id>/asset/<asset_id>",
    data = "<form>",
    rank = 3
)]
pub fn asset_update_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    asset_id: usize,
    form: Json<ApiSchema::Asset>,
) -> Result<StatusOk<ApiSchema::Asset>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .update_asset_by_id(
            asset_id,
            form.name.clone(),
            form.description.clone(),
            form.account.clone(),
            form.account_clearing.clone(),
        )?
        .into();
    Ok(StatusOk(res))
}

#[post("/repository/<repository_id>/asset/<asset_id>/remove", rank = 4)]
pub fn asset_remove_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    asset_id: usize,
) -> Result<StatusOk<ApiSchema::Asset>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .remove_asset_by_id(asset_id)?
        .into();
    Ok(StatusOk(res))
}

#[post("/repository/<repository_id>/asset/<asset_id>/restore", rank = 4)]
pub fn asset_restore_post(
    _user: Login,
    data: State<DataLoad>,
    repository_id: String,
    asset_id: usize,
) -> Result<StatusOk<ApiSchema::Asset>, ApiError> {
    let res = data
        .inner()
        .repositories
        .lock()
        .unwrap()
        .find_id_mut(&repository_id)?
        .as_mut()
        .restore_asset_by_id(asset_id)?
        .into();
    Ok(StatusOk(res))
}
