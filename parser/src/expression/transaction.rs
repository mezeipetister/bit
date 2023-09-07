use crate::{parse::Parse, token::TokenStream, Error};
use chrono::NaiveDate;

#[derive(Default, Debug)]
pub struct Transaction {
    credit: Option<String>,
    debit: Option<String>,
    amount: Option<f64>,
}

impl Transaction {
    pub(crate) fn parse(parse: &mut Parse) -> Result<Self, Error> {
        let mut target = Transaction::default();

        while let Ok(key) = &parse.next_key() {
            match key.as_str() {
                "CREDIT" => target.credit = Some(parse.next_value_bulk()?),
                "DEBIT" => target.debit = Some(parse.next_value_bulk()?),
                "AMOUNT" => target.amount = Some(parse.next_int()?),
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
