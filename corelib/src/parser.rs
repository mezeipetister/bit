use chrono::{DateTime, Utc};

enum Token {
    NOTE,
    TRANSACTION,
    ACCOUNT,
    CDATE,
    DDATE,
    IDATE,
    AMOUNT,
    PARTNER,
    DESCRIPTION,
    Number(f32),
    Text(String),
    Date(DateTime<Utc>)
}

mod v2 {
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

    fn demo(_s: String) {
        
    }
}