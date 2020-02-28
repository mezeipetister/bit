use crate::folder::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder0 {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub date_created: DateTime<Utc>,
    pub is_active: bool,
}

impl StorageObject for Folder0 {
    type ResultType = Folder0;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DeserializeError("Wrong folder schema".to_string())),
        }
    }
}
