use crate::parser::Expression;
use chrono::{Datelike, NaiveDate, Utc};
use std::{collections::HashMap, hash::Hash, ops::Deref, usize};
use thousands::Separable;

// Trimmed string representation
#[derive(Debug, Clone)]
struct TString(String);

// Only way to construct a TString
// is to convert a string slice to it
impl From<&str> for TString {
  fn from(f: &str) -> Self {
    Self(f.trim().to_string())
  }
}

impl Deref for TString {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Clone)]
pub struct Account {
  id: String,
  name: String,
}

impl Account {
  pub fn new(id: String, name: String) -> Self {
    Self { id, name }
  }
}

#[derive(Debug, Clone)]
pub struct Reference {
  id: String,
  name: Option<String>,
  idate: Option<NaiveDate>,
  cdate: NaiveDate,
  ddate: Option<NaiveDate>,
}

impl Reference {
  pub fn new(
    id: String,
    name: Option<String>,
    idate: Option<NaiveDate>,
    cdate: NaiveDate,
    ddate: Option<NaiveDate>,
  ) -> Self {
    Self {
      id,
      name,
      idate,
      cdate,
      ddate,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Event {
  id: String,
  reference_id: String,
  name: Option<String>,
  idate: Option<NaiveDate>,
  cdate: NaiveDate,
  ddate: Option<NaiveDate>,
}

impl Event {
  pub fn new(
    id: String,
    reference_id: String,
    name: Option<String>,
    idate: Option<NaiveDate>,
    cdate: NaiveDate,
    ddate: Option<NaiveDate>,
  ) -> Self {
    Self {
      id,
      reference_id,
      name,
      idate,
      cdate,
      ddate,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Transaction {
  debit: String,
  credit: String,
  event_id: String,
  cdate: NaiveDate,
  amount: i64,
}

impl Transaction {
  pub fn new(
    debit: String,
    credit: String,
    event_id: String,
    cdate: NaiveDate,
    amount: i64,
  ) -> Self {
    Self {
      debit,
      credit,
      event_id,
      cdate,
      amount,
    }
  }
}

#[derive(Debug)]
pub struct Ledger {
  accounts: Vec<Account>,
  references: HashMap<String, Reference>,
  events: HashMap<String, Event>,
  transactions: Vec<Transaction>,
  ledger_index: Vec<HashMap<String, LedgerIndexItem>>,
}

impl Ledger {
  pub fn new() -> Self {
    Self {
      accounts: Vec::new(),
      references: HashMap::new(),
      events: HashMap::new(),
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
  pub fn add_account(&mut self, account: Account) -> Result<(), String> {
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
  fn has_event(&self, event_id: &str) -> bool {
    self.events.contains_key(event_id)
  }
  pub fn add_reference(&mut self, reference: Reference) -> Result<(), String> {
    if self.has_reference(&reference.id) {
      return Err(format!("Reference ID already exist {}", &reference.id));
    }
    self.references.insert(reference.id.clone(), reference);
    Ok(())
  }
  pub fn add_event(&mut self, event: Event) -> Result<(), String> {
    if self.has_event(&event.id) {
      return Err(format!("Unknown event id {}", &event.id));
    }
    self.events.insert(event.id.clone(), event);
    Ok(())
  }
  pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
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
  pub fn get_ledger_by_date(
    &self,
    day_index: usize,
  ) -> Result<Vec<(String, LedgerIndexItem)>, String> {
    let day = self
      .ledger_index
      .get(day_index)
      .ok_or("Out of range date")?;
    let mut res = (*day)
      .clone()
      .into_iter()
      .collect::<Vec<(String, LedgerIndexItem)>>();
    res.sort_by(|x, y| x.0.cmp(&y.0));
    Ok(res)
  }
}

#[derive(Debug, Default, Clone)]
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

impl LedgerIndexItem {
  pub fn print_full(&self) -> String {
    format!(
      "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
      self.td.separate_with_spaces(),
      self.tc.separate_with_spaces(),
      self.bcd.separate_with_spaces(),
      self.bcc.separate_with_spaces()
    )
  }
}
