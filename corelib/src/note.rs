use chrono::NaiveDate;
use cli_table::{format::Justify, Cell, Style, Table};
use repository::sync::ActionPatch;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{actions::BitAction};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub amount: f32,
    pub debit: String,
    pub credit: String,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub id: Option<String>,             // Note ID
    pub partner: Option<String>,        // Partner id
    pub description: Option<String>,    // Note description
    pub cdate: Option<NaiveDate>,       // Completion date
    pub idate: Option<NaiveDate>,       // Issue date
    pub ddate: Option<NaiveDate>,       // Due date
    pub net: Option<f32>,               // Note total net value
    pub vat: Option<f32>,               // Note total vat amount
    pub gross: Option<f32>,             // Note total gross value
    pub transactions: Vec<Transaction>, // Transactions
}

impl ActionPatch<BitAction> for Note {
    const storage_id: &'static str = "note";
    fn patch(&mut self, action: BitAction, _dtime: chrono::DateTime<chrono::Utc>, _uid: &str) {
        match action {
            BitAction::NoteCreate { id } => {
                self.id = Some(id);
            }
            BitAction::NoteSet {
                partner,
                description,
                idate,
                cdate,
                ddate,
                net,
                vat,
                gross,
            } => self.set(partner, description, idate, cdate, ddate, net, vat, gross),
            BitAction::NoteUnset {
                partner,
                description,
                idate,
                cdate,
                ddate,
                net,
                vat,
                gross,
            } => self.unset(partner, description, idate, cdate, ddate, net, vat, gross),
            BitAction::NoteSetTransaction {
                debit,
                credit,
                amount,
                comment,
            } => self.set_transaction(amount, debit, credit, comment),
            _ => panic!("Just note actions can be patched to a note"),
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = vec![
            vec![
                "ID".cell(),
                (self.id.as_deref().unwrap_or("-"))
                    .cell()
                    .italic(true)
                    .bold(true)
                    .justify(Justify::Right),
            ],
            vec![
                "Partner".cell(),
                (self.partner.as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "Description".cell(),
                (self.description.as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "ISSUE date".cell(),
                (self.idate.map(|d| d.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "COMPLETION date".cell(),
                (self.cdate.map(|d| d.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "DUE date".cell(),
                (self.ddate.map(|d| d.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "Net value".cell(),
                (self.net.map(|v| v.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "VAT value".cell(),
                (self.vat.map(|v| v.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
            vec![
                "Gross value".cell(),
                (self.gross.map(|v| v.to_string()).as_deref().unwrap_or("-")).cell(),
            ],
        ]
        .table();

        let mut transactions = Vec::new();

        for (_, tr) in self.transactions.iter().enumerate() {
            let row = vec![
                // (i + 1)
                //     .cell()
                //     .italic(true)
                //     .foreground_color(Some(cli_table::Color::Yellow)),
                (&tr.debit).cell(),
                (&tr.credit).cell(),
                (&tr.amount).cell(),
                (tr.comment.as_deref().unwrap_or(""))
                    .cell()
                    .justify(Justify::Right),
            ];
            transactions.push(row);
        }

        write!(
            f,
            "{}\n\nTransactions\n{}",
            note.display().unwrap(),
            transactions
                .table()
                .title(vec![
                    // "".cell(),
                    "Debit".cell().italic(true),
                    "Credit".cell().italic(true),
                    "Amount".cell().italic(true),
                    "Comment".cell().italic(true),
                ])
                .display()
                .unwrap()
        )
    }
}

impl Note {
    pub fn new(id: Option<String>) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
    pub fn set(
        &mut self,
        partner: Option<String>,
        description: Option<String>,
        idate: Option<NaiveDate>,
        cdate: Option<NaiveDate>,
        ddate: Option<NaiveDate>,
        net: Option<f32>,
        vat: Option<f32>,
        gross: Option<f32>,
    ) {
        if partner.is_some() {
            self.partner = partner;
        }
        if description.is_some() {
            self.description = description;
        }
        if idate.is_some() {
            self.idate = idate;
        }
        if cdate.is_some() {
            self.cdate = cdate;
        }
        if ddate.is_some() {
            self.ddate = ddate;
        }
        if net.is_some() {
            self.net = net;
        }
        if vat.is_some() {
            self.vat = vat;
        }
        if gross.is_some() {
            self.gross = gross;
        }
    }
    pub fn unset(
        &mut self,
        partner: bool,
        description: bool,
        idate: bool,
        cdate: bool,
        ddate: bool,
        net: bool,
        vat: bool,
        gross: bool,
    ) {
        if partner {
            self.partner = None;
        }
        if description {
            self.description = None;
        }
        if idate {
            self.idate = None;
        }
        if cdate {
            self.cdate = None;
        }
        if ddate {
            self.ddate = None;
        }
        if net {
            self.net = None;
        }
        if vat {
            self.vat = None;
        }
        if gross {
            self.gross = None;
        }
    }
    pub fn set_transaction(
        &mut self,
        amount: f32,
        debit: String,
        credit: String,
        comment: Option<String>,
    ) {
        self.transactions.push(Transaction {
            amount,
            debit,
            credit,
            comment,
        });
    }
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn has_cdate(&self) -> bool {
        self.cdate.is_some()
    }
}
