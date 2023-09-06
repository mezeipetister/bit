use chrono::NaiveDate;

pub struct Note {
    id: String,
    partner: Option<String>,
    description: Option<String>,
    net: Option<i32>,
    vat: Option<i32>,
    gross: Option<i32>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    idate: Option<NaiveDate>,
}

impl Note {
    // pub fn from_frames(frames: Vec<Frame>) -> Self {}
}
