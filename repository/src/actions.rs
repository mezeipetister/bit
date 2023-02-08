use std::fmt::Display;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::sync::ActionExt;

#[derive(Serialize, Deserialize, Clone)]
pub enum BitActions {
  AccountCreate {
    id: String,
    name: String,
  },
  AccountRename {
    name: String,
  },
  AccountRemove,
  PartnerCreate {
    id: String,
    name: String,
  },
  PartnerRename {
    name: String,
  },
  PartnerRemove,
  NoteCreate {
    id: String,
  },
  NoteSet {
    partner: Option<String>,
    description: Option<String>,
    idate: Option<NaiveDate>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
    net: Option<f32>,
    vat: Option<f32>,
    gross: Option<f32>,
  },
  NoteUnset {
    partner: bool,
    description: bool,
    idate: bool,
    cdate: bool,
    ddate: bool,
    net: bool,
    vat: bool,
    gross: bool,
  },
  NoteSetTransaction {
    debit: String,
    credit: String,
    amount: f32,
    comment: Option<String>,
  },
}

impl Display for BitActions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BitActions::AccountCreate { id, name } => {
        write!(f, "Create account with id: {id}, name: {name}")
      }
      BitActions::AccountRename { name } => {
        write!(f, "Rename account to: {name}")
      }
      BitActions::AccountRemove => write!(f, "Remove account"),
      BitActions::PartnerCreate { id, name } => {
        write!(f, "Create partner with id: {id}, name: {name}")
      }
      BitActions::PartnerRename { name } => write!(f, "Rename partner to: {name}"),
      BitActions::PartnerRemove => write!(f, "Remove partner"),
      BitActions::NoteCreate { id } => write!(f, "Create note with id: {id}"),
      BitActions::NoteSet {
        partner,
        description,
        idate,
        cdate,
        ddate,
        net,
        vat,
        gross,
      } => write!(f, "Set note params to the followings. Partner: {partner:?}, description: {description:?}, idate: {idate:?}, cdate: {cdate:?}, ddate: {ddate:?}, net: {net:?}, vat: {vat:?}, gross: {gross:?}"),
      BitActions::NoteUnset {
        partner,
        description,
        idate,
        cdate,
        ddate,
        net,
        vat,
        gross,
      } => write!(f, "Unset note params"),
      BitActions::NoteSetTransaction {
        debit,
        credit,
        amount,
        comment,
      } => write!(f, "Note set transaction. Debit: {debit}, credit: {credit}, amount: {amount}, comment: {comment:?}"),
    }
  }
}

impl ActionExt for BitActions {
  fn display(&self) -> String {
    format!("{self}")
  }
}
