use crate::{token::TokenStream, Result};

pub mod comment;
pub mod note;
pub mod transaction;

pub fn from_str(src: &str) -> Result<()> {
    let token_stream = crate::token::parse(src)?;
    println!("{:?}", &token_stream);
    for raw_expression in token_stream {
        let mut parse = crate::parse::Parse::new(raw_expression)?;
        if let Ok(cmd) = parse.next_key() {
            match cmd.as_str() {
                "NOTE" => note::Note::parse(&mut parse)?.apply()?,
                "#" | "//" => comment::Comment::parse(&mut parse)?.apply()?,
                _ => (),
            }
        }
    }
    Ok(())
}
