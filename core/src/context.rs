use std::{path::{Path, PathBuf}, fmt::Display};

use crate::{
    fs::{cwd, is_project_cwd},
    index::IndexError,
};

#[derive(Debug)]
pub enum CtxError {
    NotProjectDir,
}

impl Display for CtxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtxError::NotProjectDir => write!(f, "Not Project Dir"),
        }
    }
}

impl From<CtxError> for IndexError {
    fn from(f: CtxError) -> Self {
        IndexError::CtxError(f)
    }
}

#[derive(Debug)]
pub struct Context {
    root_path: PathBuf,
    bitdir_path: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self, CtxError> {
        if !is_project_cwd() {
            return Err(CtxError::NotProjectDir);
        }
        let cwd = cwd();
        let bitdir_path = cwd.join(".bit");
        let root_path = cwd;
        Ok(Self {
            root_path,
            bitdir_path,
        })
    }
    // Use only for init
    pub fn new_cwd() -> Self {
        let cwd = cwd();
        let bitdir_path = cwd.join(".bit");
        let root_path = cwd;
        Self {
            root_path,
            bitdir_path,
        }
    }
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }
    pub fn bitdir_path(&self) -> &Path {
        &self.bitdir_path
    }
}
