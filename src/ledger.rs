use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, Utc};

#[derive(Debug, Clone)]
struct Account {
  id: String,
  name: String,
}

#[derive(Debug, Clone)]
struct Reference {}

#[derive(Debug, Clone)]
struct Event {}

#[derive(Debug, Clone)]
struct Transaction {
  debit: String,
  credit: String,
  amount: i64,
}

#[derive(Debug)]
pub struct Ledger {
  account_ids: HashMap<String, ()>,
  accounts: Vec<Account>,
  references: HashMap<String, Reference>,
  events: HashMap<String, Event>,
  transactions: Vec<Transaction>,
  ledger_index: Vec<Vec<LedgerIndexItem>>,
}

impl Ledger {
  pub fn new() -> Self {
    Self {
      account_ids: HashMap::new(),
      accounts: Vec::new(),
      references: HashMap::new(),
      events: HashMap::new(),
      transactions: Vec::new(),
      ledger_index: (1..NaiveDate::from_ymd(Utc::today().year(), 12, 31).ordinal())
        .into_iter()
        .map(|_| Vec::new())
        .collect(),
    }
  }
  fn has_account(&self, account_id: &str) -> bool {
    self.account_ids.contains_key(account_id)
  }
  fn add_account(&mut self, account: Account) -> Result<(), String> {
    match self.has_account(&account.id) {
      true => Err(format!("Account already exist: {}", &account.id)),
      false => {
        self.account_ids.insert(account.id.clone(), ());
        self.accounts.push(account);
        self
          .ledger_index
          .iter_mut()
          .for_each(|day| day.push(LedgerIndexItem::default()));
        Ok(())
      }
    }
  }
  fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
    // Check tr debit account
    if !self.has_account(&transaction.debit) {
      return Err(format!(
        "Unknown account ID {} for debit",
        &transaction.debit
      ));
    }
    // Check tr credit account
    if !self.has_account(&transaction.credit) {
      return Err(format!(
        "Unknown account ID for credit {}",
        &transaction.credit
      ));
    }
    // Add transaction to vec
    self.transactions.push(transaction.clone());
    // Add transaction to index
    todo!()
  }
}

#[derive(Debug, Default)]
pub struct LedgerIndexItem {
  bod: i64,
  boc: i64,
  td: i64,
  tc: i64,
  bcd: i64,
  bcc: i64,
}
