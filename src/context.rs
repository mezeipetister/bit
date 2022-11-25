use std::{
    env::args,
    path::{Path, PathBuf},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Context {
    bit_version: String,
    username: String,
    current_dir: PathBuf,
    is_project_path: bool,
    current_project_path: Option<PathBuf>,
    args_raw: Vec<String>,
    args: String,
}

impl Context {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_project_path = get_project_dir(&current_dir);
        let args_raw = std::env::args().collect::<Vec<String>>();
        let args = match args_raw.len() > 1 {
            true => args_raw[1..].join(" "),
            false => "".to_string(),
        };
        Self {
            bit_version: VERSION.to_string(),
            username: "mezeipetister".to_string(),
            current_dir: current_dir,
            is_project_path: current_project_path.is_ok(),
            current_project_path: current_project_path.map(|p| Some(p)).unwrap_or(None),
            args_raw,
            args,
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
    pub fn current_project_path(&self) -> Option<&PathBuf> {
        self.current_project_path.as_ref()
    }
    pub fn is_project_path(&self) -> bool {
        self.is_project_path
    }
    pub fn bit_version(&self) -> &str {
        &self.bit_version
    }
    pub fn args_raw(&self) -> &Vec<String> {
        &self.args_raw
    }
    pub fn args(&self) -> &str {
        &self.args
    }
}

// Try to get Yo project root path
fn get_project_dir(dir: &Path) -> Result<PathBuf, String> {
    let p = dir.join(".bit");
    match p.exists() && p.is_dir() {
        true => Ok(dir.to_path_buf()),
        false => get_project_dir(
            dir.parent()
                .ok_or("Given directory is not inside a BIT Project".to_string())?,
        ),
    }
}
