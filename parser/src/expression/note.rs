use crate::{parse::Parse, token::TokenStream, Error};
use chrono::NaiveDate;

#[derive(Default, Debug)]
pub struct Note {
    id: Option<String>,
    partner: Option<String>,
    description: Option<String>,
    net: Option<i64>,
    vat: Option<i64>,
    gross: Option<i64>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    idate: Option<NaiveDate>,
}

impl Note {
    pub(crate) fn parse(parse: &mut Parse) -> Result<Self, Error> {
        let mut note = Note::default();

        loop {
            match &parse.next_bytes()?[..] {
                b"ID" => note.id = Some(parse.next_string()?),
                b"PARTNER" => note.partner = Some(parse.next_string()?),
                b"DESCRIPTION" => note.description = Some(parse.next_string()?),
                b"NET" => note.net = Some(parse.next_int()?),
                b"VAT" => note.vat = Some(parse.next_int()?),
                b"GROSS" => note.gross = Some(parse.next_int()?),
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
