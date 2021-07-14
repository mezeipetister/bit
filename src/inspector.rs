use std::{borrow::Borrow, path::Path};

use chrono::NaiveDate;

use crate::{
  ledger::{self, Ledger},
  parser::{self, Expression, ModeExp},
};

#[derive(Debug)]
pub struct Inspector {
  expressions: Vec<Expression>,
  mode: Option<ModeExp>,
  reference_id: Option<String>,
  event_id: Option<String>,
  cdate: Option<NaiveDate>,
}

impl Inspector {
  pub fn init_from_file(file_path: &Path) -> Result<Self, String> {
    let res = Inspector {
      expressions: parser::parse_file(file_path)?,
      mode: None,
      reference_id: None,
      event_id: None,
      cdate: None,
    };
    Ok(res)
  }
  pub fn process_to_ledger(mut self, ledger: &mut Ledger) -> Result<(), String> {
    for exp in self.expressions {
      match exp {
        Expression::DocComment(_) => (), // Do nothing
        // Try to set mode
        Expression::Mode(mode_exp) => match self.mode {
          Some(_) => return Err("Mode already set".to_string()),
          None => self.mode = Some(mode_exp),
        },
        Expression::Account(account_exp) => {
          match self.mode {
            Some(ModeExp::Account) => {
              // Push account
              ledger.add_account(ledger::Account::new(account_exp.id, account_exp.name))?;
            }
            _ => return Err("Cannot define account when your mode is not account".to_string()),
          }
        }
        Expression::Transaction(transaction_exp) => {
          // Define cdate
          let cdate = match transaction_exp.cdate {
            Some(cdate) => cdate,
            None => self.cdate.ok_or("No cdate set before".to_string())?,
          };

          // Define event id
          let event_id = match transaction_exp.event_id {
            Some(eid) => eid,
            None => self
              .event_id
              .as_deref()
              .ok_or("No event id set before".to_string())?
              .to_string(),
          };

          ledger.add_transaction(ledger::Transaction::new(
            transaction_exp.debit,
            transaction_exp.credit,
            event_id,
            cdate,
            transaction_exp.amount,
          ))?;
        }
        Expression::Reference(reference_exp) => ledger.add_reference(ledger::Reference::new(
          reference_exp.id,
          reference_exp.name,
          reference_exp.idate,
          reference_exp.cdate,
          reference_exp.ddate,
        ))?,
        Expression::Event(event_exp) => {
          let cdate = match event_exp.cdate {
            Some(cdate) => cdate,
            None => self.cdate.ok_or("No cdate set before".to_string())?,
          };
          ledger.add_event(ledger::Event::new(
            event_exp.id,
            event_exp.reference_id,
            event_exp.name,
            event_exp.idate,
            cdate,
            event_exp.ddate,
          ))?;
        }
      }
    }
    Ok(())
  }
}
