// Copyright (C) 2020 peter
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

use crate::error::Error;
pub use crate::model::*;
pub use crate::prelude::*;
pub use chrono::prelude::*;

impl Project {
    pub fn new(name: String, description: String, created_by: String) -> Self {
        Project {
            id: generate_id(5),
            name,
            description,
            is_enabled: true,
            created_by,
            date_created: Utc::now(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn update(&mut self, name: String, description: String, is_enabled: bool) -> Self {
        self.name = name;
        self.description = description;
        self.is_enabled = is_enabled;
        self.clone()
    }
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
}

impl Repository {
    pub fn add_project(
        &mut self,
        name: String,
        description: String,
        created_by: String,
    ) -> AppResult<Project> {
        let p = Project::new(name, description, created_by);
        match self.check_project_id_available(p.get_id()) {
            true => {
                self.projects.push(p.clone());
                return Ok(p);
            }
            false => {
                return Err(Error::BadRequest(
                    "Hiba a projekt létrehozásakor. ID már foglalt!".to_string(),
                ))
            }
        }
    }
    pub fn check_project_id_available(&self, id: &str) -> bool {
        for project in &self.projects {
            if project.get_id() == id {
                return false;
            }
        }
        true
    }
    pub fn get_projects(&self) -> Vec<Project> {
        self.projects
            .iter()
            .map(|p| p.clone())
            .collect::<Vec<Project>>()
    }
    pub fn get_projects_enabled(&self) -> Vec<Project> {
        self.projects
            .iter()
            .filter(|p| p.is_enabled())
            .map(|p| p.clone())
            .collect::<Vec<Project>>()
    }
    pub fn get_projects_disabled(&self) -> Vec<Project> {
        self.projects
            .iter()
            .filter(|p| !p.is_enabled())
            .map(|p| p.clone())
            .collect::<Vec<Project>>()
    }
    pub fn get_project_by_id(&self, id: &str) -> AppResult<Project> {
        for project in &self.projects {
            if project.get_id() == id {
                return Ok(project.clone());
            }
        }
        Err(Error::BadRequest(
            "A megadott project ID nem létezik.".to_string(),
        ))
    }
    pub fn remove_project_by_id(&mut self, id: &str) -> AppResult<Project> {
        if let Some(position) = &self.projects.iter().position(|p| p.get_id() == id) {
            return Ok(self.projects.remove(*position));
        }
        Err(Error::BadRequest("Project id not found".to_string()))
    }
    pub fn update_project(
        &mut self,
        id: &str,
        name: String,
        description: String,
        is_enabled: bool,
    ) -> AppResult<Project> {
        for project in &mut self.projects {
            if project.get_id() == id {
                project.update(name, description, is_enabled);
                return Ok(project.clone());
            }
        }
        Err(Error::BadRequest("Project id not found".to_string()))
    }
    pub fn enable_project(&mut self, id: &str) -> AppResult<Project> {
        for project in &mut self.projects {
            if project.get_id() == id {
                project.enable();
                return Ok(project.clone());
            }
        }
        Err(Error::BadRequest("Project id not found".to_string()))
    }
    pub fn disable_project(&mut self, id: &str) -> AppResult<Project> {
        for project in &mut self.projects {
            if project.get_id() == id {
                project.disable();
                return Ok(project.clone());
            }
        }
        Err(Error::BadRequest("Project id not found".to_string()))
    }
}
