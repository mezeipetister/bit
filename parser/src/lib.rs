



pub mod expression;
pub mod parse;
pub mod token;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
