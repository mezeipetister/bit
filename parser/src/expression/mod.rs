use crate::{token::TokenStream, Result};

pub mod note;
pub mod transaction;

pub fn from_str(src: &str) -> Result<()> {
    let token_stream = crate::token::parse(src)?;
    for raw_expression in token_stream {
        let mut parse = crate::parse::Parse::new(raw_expression)?;
        let cmd = parse.next_bytes()?;
        match &cmd[..] {
            b"NOTE" => note::Note::parse(&mut parse)?.apply()?,
            _ => (),
        }
    }
    Ok(())
}
