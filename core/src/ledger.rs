use chrono::{Datelike, NaiveDate, Utc};
use cli_table::{format::Justify, Cell, Style, Table};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Add,
    rc::Rc,
};
use thousands::Separable;

use crate::{
    account::Account,
    note::{Note, Transaction},
    prelude::CliError,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AccountSummary {
    /// Balance opening
    balance_opening: f32,
    /// Turnover debit
    turnover_debit: f32,
    /// Turnover credit
    turnover_credit: f32,
    /// Balance closing
    balance_closing: f32,
}

impl AccountSummary {
    fn add_debit(&mut self, value: f32) {
        self.turnover_debit += value;
    }
    fn add_credit(&mut self, value: f32) {
        self.turnover_credit += value;
    }
}

impl Add for AccountSummary {
    type Output = AccountSummary;

    fn add(self, rhs: Self) -> Self::Output {
        let new_td = self.turnover_debit + rhs.turnover_debit;
        let new_tc = self.turnover_credit + rhs.turnover_credit;
        AccountSummary {
            balance_opening: self.balance_opening,
            turnover_debit: new_td,
            turnover_credit: new_tc,
            balance_closing: self.balance_opening + new_td - new_tc,
        }
    }
}

impl Display for AccountSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // self.turnover_debit.separate_with_spaces(),
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MonthlySummary {
    accounts: Vec<(AccountSummary)>,
}

impl MonthlySummary {
    fn init(accounts: &Vec<Account>) -> Self {
        let mut res = Self { accounts: vec![] };
        accounts
            .iter()
            .for_each(|_| res.accounts.push(AccountSummary::default()));
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ledger {
    // Store accounts here again
    accounts: Vec<Account>,
    // 2D array; 12month;
    // Monthly results
    ledger: [MonthlySummary; 12],
    // If
    should_update: bool,
    // Total notes we tried to use
    total_notes: i32,
    // Total successfully processed notes
    processed_notes: i32,
    // These notes had issues to process
    notes_with_error: Vec<String>,
}

impl Ledger {
    pub fn get(
        &mut self,
        accounts: &Vec<Account>,
        notes: &Vec<Note>,
        month: Option<u32>,
    ) -> Result<&MonthlySummary, CliError> {
        // Try to get query month ID
        let month_index = match month {
            Some(i) => match i {
                x if x >= 1 && x <= 12 => i - 1,
                _ => {
                    return Err(CliError::Error(
                        "Wrong ledger query month. 1-12".to_string(),
                    ))
                }
            },
            None => Utc::now().month0(),
        };
        // Run self test
        self.check_should_update(accounts, notes);
        // Clear itself if update needed;
        if self.should_update {
            // Clear local accounts
            self.accounts.clear();
            // Set local accounts with latest available accounts
            accounts
                .iter()
                .for_each(|a| self.accounts.push(a.to_owned()));
            // Clear ledger
            let monthly_summary = MonthlySummary::init(accounts);
            (0..11)
                .into_iter()
                .for_each(|i| self.ledger[i] = monthly_summary.clone());
            // Set total notes to 0;
            self.total_notes = 0;
            // Set processed_notes to 0;
            self.processed_notes = 0;
            // Set notes with error to 0;
            self.notes_with_error = vec![];
            // Update
            let data = self.calculate_year(accounts, notes);
            println!("{:?}", data);
        }
        // Get
        let res = &self.ledger[month_index as usize];
        // Return result
        Ok(res)
    }
    // Set for update manually
    pub fn set_should_update(&mut self) {
        self.should_update = true;
    }
    fn check_should_update(&mut self, accounts: &Vec<Account>, notes: &Vec<Note>) {
        // Check if categories are the same
        if &self.accounts != accounts {
            self.should_update = true;
        }
    }
    fn calculate_year(
        &mut self,
        accounts: &Vec<Account>,
        notes: &Vec<Note>,
    ) -> [Vec<AccountSummary>; 12] {
        // Init account hash lookup table
        let mut account_lookup: HashMap<String, usize> = HashMap::new();
        accounts.iter().enumerate().for_each(|(i, a)| {
            account_lookup.insert(a.id.to_string(), i);
        });

        // Init data to calculate width
        let mut data: [Vec<AccountSummary>; 12] = Default::default();

        // Init accounts
        (0..11).into_iter().for_each(|i| {
            accounts
                .iter()
                .for_each(|_| data[i].push(AccountSummary::default()));
        });

        // Iter over all notes
        notes.iter().for_each(|n| {
            // Check if we can process it
            // If has cdate and ID
            match (n.cdate.map(|d| d.month0()), &n.id) {
                (Some(month0_index), Some(id)) => {
                    n.transactions.iter().for_each(|tr| {
                        let debit_account_index: usize =
                            *account_lookup.get(&tr.debit).expect("Unkown account id");
                        let credit_account_index: usize =
                            *account_lookup.get(&tr.credit).expect("Unknown account id");
                        data[month0_index as usize][debit_account_index].add_debit(tr.amount);
                        data[month0_index as usize][credit_account_index].add_credit(tr.amount);
                    });
                }
                // Add ID to error vector if has no cdate or ID
                _ => self
                    .notes_with_error
                    .push(n.id.as_deref().unwrap_or("no_id").to_string()),
            }
        });
        // Return result
        data
    }
}
