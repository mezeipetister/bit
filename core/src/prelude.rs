use std::{fmt::Display, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum CliError {
    Error(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Error(e) => write!(f, "{}", e),
        }
    }
}

pub mod path_helper {
    use std::path::PathBuf;

    use crate::context::Context;
    pub fn index(ctx: &Context) -> PathBuf {
        ctx.root_path().join("BitIndex")
    }
    pub fn blob_database(ctx: &Context) -> PathBuf {
        ctx.bitdir_path().join("blob_database")
    }
}

pub trait CliDisplay
where
    Self: Sized,
{
    fn display(&self, f: &mut impl Write) -> Result<(), std::io::Error>;
    fn print(&self) -> Result<(), std::io::Error> {
        let mut writer = std::io::stdout().lock();
        self.display(&mut writer)?;
        Ok(())
    }
}

impl CliDisplay for String {
    fn display(&self, f: &mut impl Write) -> Result<(), std::io::Error> {
        write!(f, "{}", self)
    }
}

impl CliDisplay for () {
    fn display(&self, f: &mut impl Write) -> Result<(), std::io::Error> {
        write!(f, "Nothing")
    }
}
