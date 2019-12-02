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

use crate::error::Error::*;
use crate::prelude::*;
use chrono::prelude::*;
use rand::Rng;
use storaget::*;

pub trait Transaction: StorageObject {
    fn get_subject(&self) -> String;
    fn get_debit_credit(&self) -> (String, String);
    fn get_debit(&self) -> String;
    fn get_credit(&self) -> String;
    fn get_amount(&self) -> u32;
    fn get_date_created(&self) -> NaiveDateTime;
    fn get_date_settlement(&self) -> NaiveDate;
    fn get_created_by(&self) -> String;
}

pub fn create_new_id() -> AppResult<String> {
    let mut rng = rand::thread_rng();
    let mut id = String::new();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789"
        .to_owned()
        .chars()
        .collect();
    for _ in 0..20 {
        let random_ch = match chars.get(rng.gen_range(0, chars.len())) {
            Some(ch) => ch,
            None => {
                return Err(InternalError(
                    "Error while generating random password!".into(),
                ))
            }
        };
        id.push(*random_ch);
    }
    Ok(id)
}
