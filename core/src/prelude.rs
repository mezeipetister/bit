use std::{error::Error, fmt::Display};

use serde::Deserialize;
use tokio::fs::File;

#[macro_export]
macro_rules! commands {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn CommandExt>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            temp_vec
        }
    };
}

pub type BitResult<T> = Result<T, BitError>;

#[derive(Debug)]
pub struct BitError {
    msg: String,
}

impl<T> From<T> for BitError
where
    T: Error,
{
    fn from(e: T) -> Self {
        BitError::new(e)
    }
}

impl BitError {
    pub fn new<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            msg: format!("{}", msg),
        }
    }
}
