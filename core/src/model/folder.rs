// Copyright (C) 2020 Peter Mezei
//
// This file is part of Bermuda.
//
// Bermuda is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Bermuda is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Bermuda.  If not, see <http://www.gnu.org/licenses/>.

use crate::folder::*;
use crate::model::history::folder::folder0::Folder0;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder {
    /**
     * Unique ID
     * Auto generated
     */
    id: String,
    /**
     * Folder name
     */
    name: String,
    /**
     * Folder description
     */
    description: String,
    /**
     * Created by userid
     */
    created_by: String,
    /**
     * Date created
     */
    date_created: DateTime<Utc>,
    /**
     * Logical delete option
     * If its deleted its value false
     * otherwise its true
     */
    is_active: bool,
}

impl Folder {
    pub fn new(created_by: String, name: String, description: String) -> Self {
        Folder {
            id: generate_folder_id(),
            name,
            description,
            created_by,
            date_created: Utc::now(),
            is_active: true,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
}

impl StorageObject for Folder {
    type ResultType = Folder;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Ok(Folder0::try_from(from)?.into()),
        }
    }
}

impl From<Folder0> for Folder {
    fn from(from: Folder0) -> Self {
        Folder {
            id: from.id,
            name: from.title,
            description: from.description,
            created_by: from.created_by,
            date_created: from.date_created,
            is_active: from.is_active,
        }
    }
}
