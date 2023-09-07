use crate::{token::TokenStream, Result};

pub mod account;
pub mod comment;
pub mod note;
pub mod transaction;

pub fn from_str(src: &str) -> Result<()> {
    let token_stream = crate::token::parse(src)?;
    for raw_expression in token_stream {
        let mut parse = crate::parse::Parse::new(raw_expression)?;
        if let Ok(cmd) = parse.next_key() {
            match cmd.as_str() {
                "NOTE" => note::Note::parse(&mut parse)?.apply()?,
                "TRANSACTION" => transaction::Transaction::parse(&mut parse)?.apply()?,
                "ACCOUNT" => account::Account::parse(&mut parse)?.apply()?,
                "#" | "//" => comment::Comment::parse(&mut parse)?.apply()?,
                _ => (),
            }
        }
    }
    Ok(())
}
