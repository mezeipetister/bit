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

pub mod clap_parser {
    use chrono::NaiveDate;

    pub fn parse_to_naivedate(str: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(str, "%Y-%m-%d")
            .map_err(|_| format!("{} has invalid date format. (YYYY-mm-dd)", str))
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
