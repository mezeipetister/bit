use chrono::{DateTime, Utc};

use chrono::{DateTime, Utc, NaiveDate};

enum Token {
    CMD(String),
    ID(String),
    CDATE(NaiveDate),
    DDATE(NaiveDate),
    IDATE(NaiveDate),
    AMOUNT(f32),
    PARTNER(String),
    DESCRIPTION(String),
    Number(f32),
    Text(String),
    Date(DateTime<Utc>)
}

fn main() {
    let s = "Hello bello    lorem ipsum    dolorem";
    let res = s.split(" ").collect();
    println!("{:?}", res);
}