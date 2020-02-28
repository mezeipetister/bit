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
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    /**
     * Unique ID    
     */
    id: String,
    /**
     * Reference ID,
     * use as you wish
     * e.g.: invoice/contract reference ID
     */
    reference: String,
    /**
     * Folder reference
     */
    folder_id: String,
    /**
     * Document title
     */
    title: String,
    /**
     * Short description
     */
    description: String,
    /**
     * Due date, e.g.: payment date for invoice,
     * or due date for contract
     */
    due_date: Option<DateTime<Utc>>,
    /**
     * ID for enclosed document PDF
     */
    file_id: Option<String>,
    /**
     * Created by user
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

impl Document {
    pub fn new(
        created_by: String,
        reference: String,
        folder_id: String,
        title: String,
        description: String,
    ) -> Self {
        Document {
            id: generate_folder_id(),
            reference,
            folder_id,
            title,
            description,
            due_date: None,
            file_id: None,
            created_by,
            date_created: Utc::now(),
            is_active: true,
        }
    }
    pub fn get_folder(&self) -> &str {
        &self.folder_id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_reference(&mut self, reference: String) {
        self.reference = reference;
    }
    pub fn get_reference(&self) -> &str {
        &self.reference
    }
    pub fn set_folder(&mut self, folder_id: String) {
        self.folder_id = folder_id;
    }
    pub fn set_file(&mut self, file_id: Option<String>) {
        self.file_id = file_id;
    }
    pub fn set_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
    }
    pub fn get_due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
}

impl StorageObject for Document {
    type ResultType = Document;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DeserializeError(
                "document has wrong format".to_string(),
            )),
        }
    }
}
