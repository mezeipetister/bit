use std::fmt::Display;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use repository::sync::ActionExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BitAction {
    AccountRename {
        name: String,
    },
    AccountRemove,
    PartnerRename {
        name: String,
    },
    PartnerRemove,
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

impl Display for BitAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
      BitAction::AccountRename { name } => {
        write!(f, "Rename account to: {name}")
      }
      BitAction::AccountRemove => write!(f, "Remove account"),
      BitAction::PartnerRename { name } => write!(f, "Rename partner to: {name}"),
      BitAction::PartnerRemove => write!(f, "Remove partner"),
      BitAction::NoteSet {
        partner,
        description,
        idate,
        cdate,
        ddate,
        net,
        vat,
        gross,
      } => write!(f, "Set note params to the followings. Partner: {partner:?}, description: {description:?}, idate: {idate:?}, cdate: {cdate:?}, ddate: {ddate:?}, net: {net:?}, vat: {vat:?}, gross: {gross:?}"),
      BitAction::NoteUnset {
        partner,
        description,
        idate,
        cdate,
        ddate,
        net,
        vat,
        gross,
      } => write!(f, "Unset note params"),
      BitAction::NoteSetTransaction {
        debit,
        credit,
        amount,
        comment,
      } => write!(f, "Note set transaction. Debit: {debit}, credit: {credit}, amount: {amount}, comment: {comment:?}"),
    }
    }
}

impl ActionExt for BitAction {
    fn display(&self) -> String {
        format!("{self}")
    }
}
