use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    completion_date: NaiveDate,
    amount: i64,
    debit: String,
    credit: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    transactions: Vec<Transaction>,
}

impl Note {
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
