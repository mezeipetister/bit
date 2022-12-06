use std::{error::Error, fmt::Display};

use crate::message::{Message, ToMessage};

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
    fn to_message(self) -> Message {
        let status = match self {
            BitError::Msg(_) => crate::message::Status::Internal,
            BitError::ClientVersionError => crate::message::Status::VersionError,
            BitError::Unauthorized => crate::message::Status::UnAuthorized,
            BitError::BehindRemote => crate::message::Status::BehindRemote,
            BitError::Internal(_) => crate::message::Status::Internal,
        };
        Message::new_response(status)
    }
}
