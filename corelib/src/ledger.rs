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

impl<'a> Add for &'a AccountSummary {
    type Output = AccountSummary;

    fn add(self, rhs: Self) -> Self::Output {
        let new_td = rhs.turnover_debit;
        let new_tc = rhs.turnover_credit;
        AccountSummary {
            balance_opening: self.balance_closing,
            turnover_debit: new_td,
            turnover_credit: new_tc,
            balance_closing: self.balance_closing + new_td - new_tc,
        }
    }
}

#[derive(Debug)]
pub struct MonthlySummary<'a> {
    accounts: &'a Vec<Account>,
    ledger: &'a Vec<AccountSummary>,
}

impl<'a> MonthlySummary<'a> {
    fn new(accounts: &'a Vec<Account>, ledger: &'a Vec<AccountSummary>) -> Self {
        Self { accounts, ledger }
    }
}

impl<'a> Display for MonthlySummary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // self.turnover_debit.separate_with_spaces(),
        let mut transactions = Vec::new();
        // println!("{self:?}");
        // unimplemented!();
        for (account_index, account_summary) in self.ledger.iter().enumerate() {
            let row = vec![
                (&self.accounts[account_index].id).cell(),
                (textwrap::fill(&self.accounts[account_index].name, 20)).cell(),
                (account_summary.balance_opening.separate_with_underscores()).cell(),
                (account_summary.turnover_debit.separate_with_underscores()).cell(),
                (account_summary.turnover_credit.separate_with_underscores()).cell(),
                (account_summary.balance_closing.separate_with_underscores()).cell(),
            ];
            transactions.push(row);
        }
        let res = transactions
            .table()
            .title(vec![
                "Account\nID".cell().italic(true),
                "Account\nname".cell().italic(true),
                "Balance\nopening".cell().italic(true),
                "Debit\nturnover".cell().italic(true),
                "Credit\nturnover".cell().italic(true),
                "Balance\nclosing".cell().italic(true),
            ])
            .display()
            .unwrap();
        write!(f, "{res}")
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ledger {
    // Store accounts here again
    accounts: Vec<Account>,
    // 2D array; 12month;
    // Monthly results
    ledger: [Vec<AccountSummary>; 12],
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
    pub fn get<'a>(
        &'a mut self,
        accounts: &'a Vec<Account>,
        notes: &'a Vec<Note>,
        month: Option<u32>,
    ) -> Result<MonthlySummary<'a>, CliError> {
        // Try to get query month ID
        let month_index = match month {
            Some(i) => {
                assert!(i > 0, "Month number must be between 1-12");
                i - 1
            }
            None => Utc::now().month0(),
        };
        // Run self test
        self.check_should_update(accounts, notes);
        // Clear itself if update needed;
        // if self.should_update {
        if true {
            // Clear local accounts
            self.accounts.clear();
            // Set local accounts with latest available accounts
            accounts
                .iter()
                .for_each(|a| self.accounts.push(a.to_owned()));
            // Clear ledger
            self.ledger = Default::default();
            // Set total notes to 0;
            self.total_notes = 0;
            // Set processed_notes to 0;
            self.processed_notes = 0;
            // Set notes with error to 0;
            self.notes_with_error = vec![];
            // Update
            let data = self.calculate_year(accounts, notes);
            // Set ledger
            // Set monthly data
            (0..12).into_iter().for_each(|month0| {
                if month0 == 0 {
                    data[0].iter().for_each(|a| {
                        let first_account_summary = AccountSummary {
                            balance_opening: a.balance_opening,
                            turnover_debit: a.turnover_debit,
                            turnover_credit: a.turnover_credit,
                            balance_closing: a.balance_closing + a.turnover_debit
                                - a.turnover_credit,
                        };
                        self.ledger[0].push(a.to_owned());
                    });
                } else {
                    data[month0]
                        .iter()
                        .enumerate()
                        .for_each(|(account0, account_summary)| {
                            let d: AccountSummary =
                                &self.ledger[month0 - 1][account0] + account_summary;
                            self.ledger[month0].push(d);
                        });
                }
            });
            self.should_update = false;
        }
        // Get
        let res = &self.ledger[month_index as usize];
        // Return monthly summary
        Ok(MonthlySummary::new(accounts, res))
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
        (0..12).into_iter().for_each(|i| {
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
