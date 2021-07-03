use std::{collections::HashMap, usize};

use chrono::{Datelike, NaiveDate, Utc};

#[derive(Debug, Clone)]
struct Account {
  id: String,
  name: String,
}

#[derive(Debug, Clone)]
struct Reference {
  id: String,
  name: Option<String>,
  idate: Option<NaiveDate>,
  cdate: NaiveDate,
  ddate: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
struct Event {
  reference_id: String,
  name: Option<String>,
  idate: Option<NaiveDate>,
  cdate: NaiveDate,
  ddate: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
struct Transaction {
  debit: String,
  credit: String,
  // event_id: u32,
  cdate: NaiveDate,
  amount: i64,
}

#[derive(Debug)]
pub struct Ledger {
  accounts: Vec<Account>,
  references: HashMap<String, Reference>,
  events: Vec<Event>,
  transactions: Vec<Transaction>,
  ledger_index: Vec<HashMap<String, LedgerIndexItem>>,
}

impl Ledger {
  pub fn new() -> Self {
    Self {
      accounts: Vec::new(),
      references: HashMap::new(),
      events: Vec::new(),
      transactions: Vec::new(),
      ledger_index: (0..NaiveDate::from_ymd(Utc::today().year(), 12, 31).ordinal0())
        .into_iter()
        .map(|_| HashMap::new())
        .collect(),
    }
  }
  fn has_account(&self, account_id: &str) -> bool {
    match self.ledger_index.get(0) {
      Some(x) => x.contains_key(account_id),
      None => false,
    }
  }
  fn add_account(&mut self, account: Account) -> Result<(), String> {
    match self.has_account(&account.id) {
      true => Err(format!("Account already exist: {}", &account.id)),
      false => {
        self.ledger_index.iter_mut().for_each(|day| {
          day.insert(account.id.clone(), LedgerIndexItem::default());
        });
        self.accounts.push(account);
        Ok(())
      }
    }
  }
  fn has_reference(&self, reference_id: &str) -> bool {
    self.references.contains_key(reference_id)
  }
  fn add_reference(&mut self, reference: Reference) -> Result<(), String> {
    if self.has_reference(&reference.id) {
      return Err(format!("Reference ID already exist {}", &reference.id));
    }
    self.references.insert(reference.id.clone(), reference);
    Ok(())
  }
  fn add_event(&mut self, event: Event) -> Result<(), String> {
    if self.has_reference(&event.reference_id) {
      return Err(format!("Unknown event id {}", &event.reference_id));
    }
    self.events.push(event);
    Ok(())
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
    let last_day = self.ledger_index.len();
    let completion_day = transaction.cdate.ordinal0() as usize;

    // Process completion day debit
    let index = &mut self.ledger_index[completion_day];
    match index.get_mut(&transaction.debit) {
      Some(index_item) => {
        index_item.td += transaction.amount;
        index_item.bcd += transaction.amount;
      }
      None => (),
    }
    match index.get_mut(&transaction.credit) {
      Some(index_item) => {
        index_item.tc += transaction.amount;
        index_item.bcc += transaction.amount;
      }
      None => (),
    }

    (completion_day + 1..last_day)
      .into_iter()
      .for_each(|day_number| {
        let item = &mut self.ledger_index[day_number];

        // Process debit
        match item.get_mut(&transaction.debit) {
          Some(index_item) => {
            index_item.bod += transaction.amount;
            index_item.bcd += transaction.amount;
          }
          None => (),
        }

        // Process credit
        match item.get_mut(&transaction.credit) {
          Some(index_item) => {
            index_item.boc += transaction.amount;
            index_item.bcc += transaction.amount;
          }
          None => (),
        }
      });
    Ok(())
  }
}

#[derive(Debug, Default)]
pub struct LedgerIndexItem {
  /// Balance opening debit
  bod: i64,
  /// Balance opening credit
  boc: i64,
  /// Turnover debit
  td: i64,
  /// Turnover credit
  tc: i64,
  /// Balance closing debit
  bcd: i64,
  /// Balance closing credit
  bcc: i64,
}
