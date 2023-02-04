use std::fmt::Display;

use chrono::NaiveDate;
use cli_table::{format::Justify, Cell, Style, Table};
use serde::{Deserialize, Serialize};

use crate::{account::Account, prelude::CliError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    amount: f32,
    debit: String,
    credit: String,
    comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub id: Option<String>,         // Note ID
    partner: Option<String>,        // Partner id
    description: Option<String>,    // Note description
    idate: Option<NaiveDate>,       // Issue date
    cdate: Option<NaiveDate>,       // Completion date
    ddate: Option<NaiveDate>,       // Due date
    net: Option<f32>,               // Note total net value
    vat: Option<f32>,               // Note total vat amount
    gross: Option<f32>,             // Note total gross value
    transactions: Vec<Transaction>, // Transactions
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = vec![
            vec![
                "ID".cell(),
                (self.id.as_deref().unwrap_or("-"))
                    .cell()
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

        for (i, tr) in self.transactions.iter().enumerate() {
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
    pub fn set_transaction(
        &mut self,
        amount: f32,
        debit: Account,
        credit: Account,
        comment: Option<String>,
    ) -> Result<(), CliError> {
        self.transactions.push(Transaction {
            amount,
            debit: debit.id.to_owned(),
            credit: credit.id.to_owned(),
            comment,
        });
        Ok(())
    }
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
