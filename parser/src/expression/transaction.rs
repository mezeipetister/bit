use chrono::NaiveDate;

use crate::{parse::Parse, Error};

#[derive(Default, Debug)]
pub struct Transaction {
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    idate: Option<NaiveDate>,
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
                "CDATE" => target.cdate = Some(parse.next_date()?),
                "DDATE" => target.ddate = Some(parse.next_date()?),
                "IDATE" => target.idate = Some(parse.next_date()?),
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
