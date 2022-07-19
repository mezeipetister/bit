use serde::{Deserialize, Serialize};
use std::path::Path;

/// BIT Config
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub name: String,
    pub description: Option<String>,
    pub year: i32,
    pub currency: String,
    pub bit_version: u32,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependencies {
    pub accounts_path: String,
    pub docs_path: String,
    pub notes_path: String,
}

impl Settings {
    pub fn try_read(project_root_path: &Path) -> Result<Self, String> {
        // Try load config file
        let cfg_file = project_root_path.join("Bit.toml");
        // Check if it exist
        match cfg_file.exists() || cfg_file.is_file() {
            true => (),
            false => return Err("No config file found!".to_string()),
        }
        // Try to read its content
        let cfg_content = std::fs::read_to_string(&cfg_file)
            .map_err(|_| "Cannot read config.toml content".to_string())?;
        // Try to deserialize its contetn
        let settings: Settings = toml::from_str(&cfg_content)
            .map_err(|_| "Error while deserialize config.toml".to_string())?;

        let accounts_path = project_root_path.join(&settings.dependencies.accounts_path);
        // Check depdendencies
        if !accounts_path.exists() && !accounts_path.is_file() {
            return Err("Accounts file does not exist!".to_string());
        }

        let notes_path = project_root_path.join(&settings.dependencies.notes_path);
        if !notes_path.exists() && !notes_path.is_dir() {
            return Err("NOTEs path not exist or not a folder!".to_string());
        }

        Ok(settings)
    }
}
