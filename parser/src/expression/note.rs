use crate::{parse::Parse, Error};
use chrono::NaiveDate;

#[derive(Default, Debug)]
pub struct Note {
    id: Option<String>,
    partner: Option<String>,
    description: Option<String>,
    net: Option<f64>,
    vat: Option<f64>,
    gross: Option<f64>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    idate: Option<NaiveDate>,
}

impl Note {
    pub(crate) fn parse(parse: &mut Parse) -> Result<Self, Error> {
        let mut note = Note::default();

        while let Ok(key) = &parse.next_key() {
            match key.as_str() {
                "ID" => note.id = Some(parse.next_value_bulk()?),
                "PARTNER" => note.partner = Some(parse.next_value_bulk()?),
                "DESCRIPTION" => note.description = Some(parse.next_value()?),
                "NET" => note.net = Some(parse.next_int()?),
                "VAT" => note.vat = Some(parse.next_int()?),
                "GROSS" => note.gross = Some(parse.next_int()?),
                "CDATE" => note.cdate = Some(parse.next_date()?),
                "DDATE" => note.ddate = Some(parse.next_date()?),
                "IDATE" => note.idate = Some(parse.next_date()?),
                _ => return Err("Unknown parameter".into()),
            }
        }

        Ok(note)
    }

    pub(crate) fn apply(&self) -> Result<(), Error> {
        println!("{:?}", self);
        Ok(())
    }
}
