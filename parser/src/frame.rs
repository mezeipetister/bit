use bytes::Buf;
use std::{fmt, io::Cursor, num::TryFromIntError, string::FromUtf8Error};

#[derive(Debug)]
pub enum Token {
    Key(String),
    Value(String),
}

impl Token {
    fn from_bytes(src: &[u8]) -> Result<Token, Error> {
        let raw_token = String::from_utf8(src.to_vec())?;
        match raw_token.chars().all(char::is_uppercase) {
            true => Ok(Token::Key(raw_token)),
            false => Ok(Token::Value(raw_token)),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,
    EmptyBlock,
    Done,
    /// Invalid message encoding
    Other(String),
}

pub fn parse(src: &str) -> Result<Vec<Vec<Token>>, Error> {
    let mut src = Cursor::new(src.as_bytes());

    let mut blocks = Vec::new();

    while let Ok(block) = get_block(&mut src) {
        blocks.push(block);
    }

    let mut res = Vec::new();

    for block in blocks {
        res.push(tokenize_block(block)?);
    }

    Ok(res)
}

fn tokenize_block<'a>(src: &'a [u8]) -> Result<Vec<Token>, Error> {
    let mut src = Cursor::new(src);
    let mut tokens = Vec::new();

    while let Ok(token) = get_token(&mut src) {
        println!("{:?}", &token);
        tokens.push(token);
    }

    Ok(tokens)
}

/// Find a block of code
fn get_block<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\n' && src.get_ref()[i + 1] != b' ' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 1) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i + 1]);
        }
    }

    Err(Error::Done)
}

fn get_token(src: &mut Cursor<&[u8]>) -> Result<Token, Error> {
    Token::from_bytes(next_token_bytes(src)?)
}

fn next_token_bytes<'a>(src: &'a mut Cursor<&[u8]>) -> Result<&'a [u8], Error> {
    let len = src.get_ref().len();

    if len == 0 {
        return Err(Error::Done);
    }

    let start = src.position() as usize;
    let end = len - 1;
    let mut _start = start;

    for i in start..end {
        let current_byte = src.get_ref()[i];
        let next_byte = src.get_ref()[i + 1];
        if current_byte == b' ' || current_byte == b'\n' {
            _start = i + 1;
            continue;
        }
        if next_byte == b' ' || next_byte == b'\n' {
            src.set_position(i as u64 + 1);
            return Ok(&src.get_ref()[_start..i + 1]);
        }
    }

    Err(Error::Done)
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Done => "end of stream".fmt(fmt),
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
            _ => "Unknown error".fmt(fmt),
        }
    }
}
