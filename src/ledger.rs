use chrono::{Datelike, NaiveDate, Utc};
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    path::PathBuf,
};
use thousands::Separable;

use crate::note::Note;

#[derive(Default, Debug, Eq, Hash, PartialEq)]
pub struct Account {
    pub id: String,
    pub name: String,
}

impl Account {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug)]
struct Transaction {
    completion_date: NaiveDate,
    amount: i64,
    debit: String,
    credit: String,
}

#[derive(Default, Debug)]
pub struct Ledger {
    ledger_index: Vec<HashMap<String, LedgerIndexItem>>,
    note_counter: i32,
    transaction_counter: i32,
}

impl Ledger {
    pub fn init(&mut self, accounts: &HashSet<String>) {
        // Create all days in index as empty
        self.ledger_index = (0..NaiveDate::from_ymd(Utc::today().year(), 12, 31).ordinal0())
            .into_iter()
            .map(|_| HashMap::new())
            .collect();
        // Add accounts to index to each day
        accounts.iter().for_each(|account| {
            self.ledger_index.iter_mut().for_each(|day| {
                day.insert(account.clone(), LedgerIndexItem::default());
            });
        });
    }
    pub fn add_note(&mut self, note: &Note, accounts: &HashSet<String>) -> Result<(), String> {
        if note.transactions.len() == 0 {
            return Ok(());
        }
        let completion_date = note
            .completion_date
            .clone()
            .ok_or_else(|| "No completion date for note!".to_string())?;
        self.note_counter += 1;
        for transaction in &note.transactions {
            self.add_transaction(
                Transaction {
                    completion_date: completion_date,
                    amount: transaction.amount,
                    debit: transaction.debit.clone(),
                    credit: transaction.credit.clone(),
                },
                accounts,
            )?;
        }
        Ok(())
    }
    fn add_transaction(
        &mut self,
        transaction: Transaction,
        accounts: &HashSet<String>,
    ) -> Result<(), String> {
        let has_account = |account_id: &str| accounts.contains(account_id);
        // Check tr debit account
        if !has_account(&transaction.debit) {
            return Err(format!(
                "Unknown account ID {} for debit",
                &transaction.debit
            ));
        }
        // Check tr credit account
        if !has_account(&transaction.credit) {
            return Err(format!(
                "Unknown account ID for credit {}",
                &transaction.credit
            ));
        }
        // Add transaction to index
        let last_day = self.ledger_index.len();
        let completion_day = transaction.completion_date.ordinal0() as usize;

        // Process completion day debit
        let index = &mut self.ledger_index[completion_day];
        // cbd => closing balance debit
        let cbd = match index.get_mut(&transaction.debit) {
            Some(index_item) => {
                index_item.td += transaction.amount;
                index_item.bc = index_item.bo + (index_item.td - index_item.tc);
                index_item.bc
            }
            None => panic!("Cannot get mutable account"),
        };
        // cbc => closing balance credit
        let cbc = match index.get_mut(&transaction.credit) {
            Some(index_item) => {
                index_item.tc += transaction.amount;
                index_item.bc = index_item.bo + (index_item.td - index_item.tc);
                index_item.bc
            }
            None => panic!("Cannot get mutable account"),
        };

        (completion_day + 1..last_day)
            .into_iter()
            .for_each(|day_number| {
                let item = &mut self.ledger_index[day_number];

                // Process debit
                match item.get_mut(&transaction.debit) {
                    Some(index_item) => {
                        index_item.bo += cbd;
                        index_item.bc += index_item.bo + index_item.td - index_item.tc;
                    }
                    None => (),
                }

                // Process credit
                match item.get_mut(&transaction.credit) {
                    Some(index_item) => {
                        index_item.bo += cbc;
                        index_item.bc += index_item.bo + index_item.td - index_item.tc;
                    }
                    None => (),
                }
            });
        Ok(())
    }
    pub fn get_ledger_by_date(
        &self,
        day_index: usize,
    ) -> Result<HashMap<String, LedgerIndexItem>, String> {
        let day = self
            .ledger_index
            .get(day_index)
            .ok_or("Out of range date")?;
        Ok((*day).clone())
    }
}

#[derive(Debug, Default, Clone)]
pub struct LedgerIndexItem {
    /// Balance opening
    bo: i64,
    /// Turnover debit
    td: i64,
    /// Turnover credit
    tc: i64,
    /// Balance closing
    bc: i64,
}

impl Add for LedgerIndexItem {
    type Output = LedgerIndexItem;

    fn add(self, rhs: Self) -> Self::Output {
        let new_td = self.td + rhs.td;
        let new_tc = self.tc + rhs.tc;
        LedgerIndexItem {
            bo: self.bo,
            td: new_td,
            tc: new_tc,
            bc: self.bo + new_td - new_tc,
        }
    }
}

impl LedgerIndexItem {
    pub fn print_full(&self) -> String {
        format!(
            "{0: <13} | {1: <13} | {2: <13} | {3: <13}",
            self.td.separate_with_spaces(),
            self.tc.separate_with_spaces(),
            match self.bc {
                x if x >= 0 => x.abs(),
                _ => 0,
            }
            .separate_with_spaces(),
            match self.bc {
                x if x < 0 => x.abs(),
                _ => 0,
            }
            .separate_with_spaces()
        )
    }
}

// None => yearly YYYY-01-01 - Current month
// mm => full month of current year
pub fn unzip_dates(date_str: Option<String>) -> Result<(NaiveDate, NaiveDate), String> {
    match date_str {
        Some(month) => {
            // Try to map month str to i32
            let month = month
                .parse::<u32>()
                .map_err(|_| "Wrong month number. 1-12".to_string())?;

            // Check date range
            if month < 1 || month > 12 {
                return Err("Month date should be a valid 1-12 integer".to_string());
            }

            // Define month first day
            let sdate = NaiveDate::from_ymd(Utc::today().year(), month, 1);

            // Define month last day
            let ldate = NaiveDate::from_ymd_opt(Utc::today().year(), month + 1, 1)
                .unwrap_or(NaiveDate::from_ymd(Utc::today().year() + 1, 1, 1))
                .pred();

            // Return month date range
            Ok((sdate, ldate))
        }
        None => Ok((
            NaiveDate::from_ymd(Utc::today().year(), 1, 1),
            Utc::today().naive_local(),
        )),
    }
}
