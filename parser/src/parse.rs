use std::{fmt, iter::Peekable, vec};

use chrono::NaiveDate;

use crate::token::{Token, TokenStream};

#[derive(Debug)]
pub(crate) struct Parse {
    /// Array frame iterator.
    parts: Peekable<vec::IntoIter<Token>>,
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
            parts: token_stream.tokens().into_iter().peekable(),
        })
    }

    /// Return next token
    fn next(&mut self) -> Result<Token, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, ParseError> {
        self.next()
    }

    /// Return the next entry as a string.
    ///
    /// If the next entry cannot be represented as a String, then an error is returned.
    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Token::Key(t) => Ok(t),
            Token::Value(t) => Ok(t),
        }
    }

    pub(crate) fn next_key(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Token::Key(t) => Ok(t),
            Token::Value(_) => Err(ParseError::Other("invalid argument. must be a key!".into())),
        }
    }

    pub(crate) fn next_value(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Token::Key(_) => Err(ParseError::Other(
                "invalid argument. must be a value!".into(),
            )),
            Token::Value(t) => Ok(t),
        }
    }

    pub(crate) fn next_value_bulk(&mut self) -> Result<String, ParseError> {
        let mut res: Vec<String> = vec![];

        loop {
            res.push(self.next_value()?);
            match self.parts.peek() {
                Some(token) => match token {
                    Token::Key(_) => break,
                    Token::Value(_) => (),
                },
                None => break,
            }
        }

        Ok(res.join(" "))
    }

    /// Return the next entry as an integer.
    ///
    /// This includes `Simple`, `Bulk`, and `Integer` frame types. `Simple` and
    /// `Bulk` frame types are parsed.
    ///
    /// If the next entry cannot be represented as an integer, then an error is
    /// returned.
    pub(crate) fn next_int(&mut self) -> Result<f64, ParseError> {
        self.next_string()?
            .parse::<f64>()
            .map_err(|_| ParseError::Other("Not a number".into()))
    }

    pub(crate) fn next_date(&mut self) -> Result<NaiveDate, ParseError> {
        self.next_string()?
            .parse::<NaiveDate>()
            .map_err(|_| ParseError::Other("Not a date".into()))
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
