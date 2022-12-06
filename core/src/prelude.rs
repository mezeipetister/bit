use std::{error::Error, fmt::Display};

use serde::Deserialize;
use tokio::fs::File;

use crate::sync::{Message, ToMessage};

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
pub enum BitError {
    Msg(String),
    ClientVersionError,
    Unauthorized,
    BehindRemote,
    Internal(String),
}

impl<T> From<T> for BitError
where
    T: Error,
{
    fn from(e: T) -> Self {
        BitError::Internal(e.to_string())
    }
}

impl BitError {
    pub fn new<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Msg(format!("{}", msg))
    }
}

impl ToMessage for BitError {
    fn to_message(self, ctx: &crate::context::Context) -> Message {
        let status = match self {
            BitError::Msg(_) => crate::sync::Status::Internal,
            BitError::ClientVersionError => crate::sync::Status::VersionError,
            BitError::Unauthorized => crate::sync::Status::UnAuthorized,
            BitError::BehindRemote => crate::sync::Status::BehindRemote,
            BitError::Internal(_) => crate::sync::Status::Internal,
        };
        Message::new_response(ctx, status)
    }
}
