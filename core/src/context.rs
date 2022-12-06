use std::path::{Path, PathBuf};

use crate::prelude::{BitError, BitResult};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub enum Mode {
    Client,
    Server,
    Setup,
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mode::Client, Mode::Client) => true,
            (Mode::Server, Mode::Server) => true,
            (Mode::Setup, Mode::Setup) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    bit_version: String,
    username: String,
    current_dir: PathBuf,
    remote_address: Option<String>,
    project_path: PathBuf,
    bit_data_path: PathBuf,
    mode: Mode,
    args_raw: Vec<String>,
    args: String,
}

impl Context {
    pub fn new(mode: Mode) -> BitResult<Self> {
        let current_dir = std::env::current_dir().unwrap();
        let current_project_path = get_project_dir(&current_dir)?;
        let args_raw = std::env::args().collect::<Vec<String>>();
        let args = match args_raw.len() > 1 {
            true => args_raw[1..].join(" "),
            false => "".to_string(),
        };
        let res = Self {
            bit_version: VERSION.to_string(),
            username: "mezeipetister".to_string(),
            current_dir,
            remote_address: Some("http://localhost:17017".to_string()),
            bit_data_path: current_project_path.join(".bit"),
            project_path: current_project_path,
            mode,
            args_raw,
            args,
        };
        Ok(res)
    }
    pub fn new_client() -> BitResult<Self> {
        Self::new(Mode::Client)
    }
    pub fn new_server() -> BitResult<Self> {
        Self::new(Mode::Server)
    }
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
    pub fn mode_is_server(&self) -> bool {
        self.mode == Mode::Server
    }
    pub fn mode_is_local(&self) -> bool {
        self.mode == Mode::Client
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
    pub fn current_project_path(&self) -> &PathBuf {
        &self.project_path
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
    pub fn bit_data_path(&self) -> &PathBuf {
        &self.bit_data_path
    }
    pub fn remote_address(&self) -> Option<&String> {
        self.remote_address.as_ref()
    }
}

// Try to get Yo project root path
fn get_project_dir(dir: &Path) -> BitResult<PathBuf> {
    let p = dir.join(".bit");
    match p.exists() && p.is_dir() {
        true => Ok(dir.to_path_buf()),
        false => get_project_dir(
            dir.parent()
                .ok_or(BitError::new("Given directory is not inside a BIT Project"))?,
        ),
    }
}
