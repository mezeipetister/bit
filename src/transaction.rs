use chrono::NaiveDate;
use uuid::Uuid;

pub struct Account {
  id: String,   // User defined ID
  name: String, // Human readable account name
}

impl Account {
  /// Create new account object
  pub fn new(id: String, name: String) -> Self {
    Self {
      id: id.trim().to_lowercase(),
      name: name.trim().to_string(),
    }
  }
  /// Rename
  pub fn rename(&mut self, name: String) -> &Self {
    self.name = name.trim().to_string();
    self
  }
  /// Get ID
  pub fn id(&self) -> &str {
    &self.id
  }
  /// Get Name
  pub fn name(&self) -> &str {
    &self.name
  }
}

pub struct Reference {
  id: String,               // User defined ID
  name: String,             // Short name for reference
  idate: Option<NaiveDate>, // Issued date (kiállítás dátuma)
  cdate: NaiveDate,         // Completion date
  ddate: Option<NaiveDate>, // Duedate (payment)
  events: Vec<Event>,       // Contained events
}

impl Reference {
  pub fn new(
    id: String,
    name: String,
    idate: Option<NaiveDate>,
    cdate: NaiveDate,
    ddate: Option<NaiveDate>,
  ) -> Self {
    Self {
      id: id.trim().to_lowercase(),
      name: name.trim().to_string(),
      idate,
      cdate,
      ddate,
      events: Vec::new(),
    }
  }
  pub fn id(&self) -> &str {
    &self.id
  }
  pub fn name(&self) -> &str {
    &self.name
  }
  pub fn idate(&self) -> Option<NaiveDate> {
    self.idate
  }
  pub fn cdate(&self) -> NaiveDate {
    self.cdate
  }
  pub fn ddate(&self) -> Option<NaiveDate> {
    self.ddate
  }
  pub fn events(&self) -> &Vec<Event> {
    &self.events
  }
  pub fn rename(&mut self, name: String) -> &Self {
    self.name = name.trim().to_string();
    self
  }
  pub fn add_event(&mut self, event: Event) -> Result<&Self, String> {
    self.events.push(event);
    self
  }
}

pub struct Event {
  id: u32,                        // Unique ID
  reference: String,              // Related reference
  name: Option<String>,           // Short description of the economic event
  cdate: NaiveDate,               // Completion date
  transactions: Vec<Transaction>, // Contained transactions
}

pub struct Transaction {
  id: Uuid,          // Unique ID
  reference: String, // Related reference
  event: u32,        // Related event ID
  debit: String,     // Accounted debit
  credit: String,    // Accounted credit
  amount: i32,       // Accounted amount
  cdate: NaiveDate,  // Accounted completion date
}

pub struct Index {
  accounts: Vec<Account>,
  references: Vec<Reference>,
  events: Vec<Event>,
  transactions: Vec<Transaction>,
}

pub struct LedgerItem {
  total_debit: i32,
  total_credit: i32,
  total_balance: i32,
  count_reference: i32,
  count_events: i32,
  count_transactions: i32,
}
