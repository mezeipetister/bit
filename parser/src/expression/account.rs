use crate::{parse::Parse, Error};

#[derive(Default, Debug)]
pub struct Account {
    id: Option<String>,
    name: Option<String>,
}

impl Account {
    pub(crate) fn parse(parse: &mut Parse) -> Result<Self, Error> {
        let mut target = Account::default();

        while let Ok(key) = &parse.next_key() {
            match key.as_str() {
                "ID" => target.id = Some(parse.next_value_bulk()?),
                "NAME" => target.name = Some(parse.next_value_bulk()?),
                "#" | "//" => {
                    let _ = parse.next_value_bulk()?;
                }
                _ => return Err("Unknown parameter".into()),
            }
        }

        Ok(target)
    }

    pub(crate) fn apply(&self) -> Result<(), Error> {
        println!("{:?}", self);
        Ok(())
    }
}
