use crate::error::Error::*;
use crate::prelude::*;

pub fn check_email(address: &str) -> AppResult<()> {
    if !address.contains("@") {
        return Err(InternalError("Wrong email format. Missing @.".into()));
    }
    if !address.contains(".") {
        return Err(InternalError("Wrong email format. Missing dot.".into()));
    }
    if address.len() < 4 {
        return Err(InternalError("Wrong email format. Too short.".into()));
    }
    Ok(())
}
