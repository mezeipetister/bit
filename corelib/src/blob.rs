

use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Blob {
    id: String,
    note_id: Option<String>,
    file_name: Option<String>,
    title: Option<String>,
    description: Option<String>,
}

// impl Blob {
//     pub fn new(file_name: String, file_content: Vec<u8>) -> Result<Self, CliError> {
//         let res = Self {
//             id: Uuid::new_v4(),
//             file_name,
//         };
//         Ok(res)
//     }
//     pub fn id(&self) -> Uuid {
//         self.id
//     }
//     pub fn name(&self) -> &str {
//         &self.file_name
//     }
//     pub fn content_bytes(&self) -> Result<Vec<u8>, CliError> {
//         unimplemented!();
//     }
//     pub fn open_as_temp_file(&self) -> Result<PathBuf, CliError> {
//         unimplemented!()
//     }
// }
