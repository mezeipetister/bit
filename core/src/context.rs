use std::{
    env::args,
    path::{Path, PathBuf},
};

use crate::prelude::BitResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub enum Mode {
    Local,
    Server,
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mode::Local, Mode::Local) => true,
            (Mode::Local, Mode::Server) => false,
            (Mode::Server, Mode::Local) => false,
            (Mode::Server, Mode::Server) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    bit_version: String,
    username: String,
    current_dir: PathBuf,
    is_bit_project_path: bool,
    project_path: Option<PathBuf>,
    bit_data_path: Option<PathBuf>,
    mode: Mode,
    args_raw: Vec<String>,
    args: String,
}

impl Context {
    pub fn new(mode: Mode) -> Self {
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
            is_bit_project_path: current_project_path.is_ok(),
            bit_data_path: current_project_path
                .as_ref()
                .map(|p| Some(p.join(".bit")))
                .unwrap_or(None),
            project_path: current_project_path.map(|p| Some(p)).unwrap_or(None),
            mode,
            args_raw,
            args,
        }
    }
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
    pub fn mode_is_server(&self) -> bool {
        self.mode == Mode::Server
    }
    pub fn mode_is_local(&self) -> bool {
        self.mode == Mode::Local
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
    pub fn current_project_path(&self) -> Option<&PathBuf> {
        self.project_path.as_ref()
    }
    pub fn is_bit_project_path(&self) -> bool {
        self.is_bit_project_path
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
    pub fn bit_data_path(&self) -> Option<&PathBuf> {
        self.bit_data_path.as_ref()
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
