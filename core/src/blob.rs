use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Blob {
    id: Uuid,
    file_name: String,
    file_content: Vec<u8>,
}

impl Blob {
    pub fn new(file_name: String, file_content: Vec<u8>) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_name,
            file_content,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.file_name
    }
    pub fn content_bytes(&self) -> &[u8] {
        self.file_content.as_slice()
    }
}
