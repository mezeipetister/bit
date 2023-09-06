use bytes::Bytes;
use std::{fmt, str, vec};

use crate::token::{Token, TokenStream};

#[derive(Debug)]
pub(crate) struct Parse {
    /// Array frame iterator.
    parts: vec::IntoIter<Token>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    EndOfStream,
    /// All other errors
    Other(String),
}

impl Parse {
    pub(crate) fn new(token_stream: TokenStream) -> Result<Parse, ParseError> {
        Ok(Parse {
            parts: token_stream.tokens().into_iter(),
        })
    }

    /// Return next token
    fn next(&mut self) -> Result<Token, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }

    /// Return the next entry as a string.
    ///
    /// If the next entry cannot be represented as a String, then an error is returned.
    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match String::from_utf8(self.next()?.to_bytes()) {
            Ok(text) => Ok(text),
            Err(e) => Err(ParseError::Other("UTF8 error".into())),
        }
    }

    /// Return the next entry as raw bytes.
    ///
    /// If the next entry cannot be represented as raw bytes, an error is
    /// returned.
    pub(crate) fn next_bytes(&mut self) -> Result<Vec<u8>, ParseError> {
        Ok(self.next()?.to_bytes())
    }

    /// Return the next entry as an integer.
    ///
    /// This includes `Simple`, `Bulk`, and `Integer` frame types. `Simple` and
    /// `Bulk` frame types are parsed.
    ///
    /// If the next entry cannot be represented as an integer, then an error is
    /// returned.
    pub(crate) fn next_int(&mut self) -> Result<i64, ParseError> {
        self.next_string()?
            .parse::<i64>()
            .map_err(|_| ParseError::Other("Not an integer".into()))
    }

    /// Ensure there are no more entries in the array
    pub(crate) fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::Other(
                "protocol error; expected end of frame, but there was more".into(),
            ))
        }
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "End of stream".fmt(fmt),
            ParseError::Other(e) => e.fmt(fmt),
        }
    }
}
