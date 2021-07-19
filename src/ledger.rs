use crate::parser::Expression;
use chrono::{Datelike, Duration, NaiveDate, Utc};
use std::{
  collections::HashMap,
  hash::Hash,
  ops::{Add, Deref},
  usize,
};
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
  pub id: String,
  pub name: String,
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
  pub accounts: Vec<Account>,
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
      let ldate = NaiveDate::from_ymd(Utc::today().year(), month + 1, 1) - Duration::days(1);

      // Return month date range
      Ok((sdate, ldate))
    }
    None => Ok((
      NaiveDate::from_ymd(Utc::today().year(), 1, 1),
      Utc::today().naive_local(),
    )),
  }
}
