use crate::{parse::Parse, Error};

#[derive(Default, Debug)]
pub struct Comment {
    message: String,
}

impl Comment {
    pub(crate) fn parse(parse: &mut Parse) -> Result<Self, Error> {
        let mut target = Self::default();
        target.message = parse.remaining()?;

        Ok(target)
    }

    pub(crate) fn apply(&self) -> Result<(), Error> {
        println!("{:?}", self);
        Ok(())
    }
}
