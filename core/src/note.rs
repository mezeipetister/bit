use std::path::{Path, PathBuf};

use chrono::{Local, NaiveDate};
use serde::Deserialize;
use serde_cbor::error;

use crate::{
    ledger::Account,
    parser::{Command, NoteRaw, Token, TokenKind},
};

#[derive(Default, Debug)]
pub struct Note {
    pub path: PathBuf,
    pub id: Option<String>,
    pub alias: Option<String>,
    pub docid: Option<String>,
    pub author: Option<String>,
    pub payment_kind: Option<PaymentKind>,
    pub net: i64,
    pub vat: i64,
    pub gross: i64,
    pub issue_date: Option<NaiveDate>,
    pub completion_date: Option<NaiveDate>,
    pub duedate: Option<NaiveDate>,
    pub transactions: Vec<Transaction>,
    pub accounts: Vec<Account>,
}

impl Note {
    pub fn from_file(path: &Path, is_account_file: bool) -> Result<Self, String> {
        let raw = NoteRaw::from_file(path)?;
        Self::from_raw_note(raw, is_account_file)
    }
    pub fn from_raw_note(raw: NoteRaw, is_account_file: bool) -> Result<Self, String> {
        let mut note = Note::default();
        note.path = raw.file_path().to_owned();
        for line in raw.lines() {
            // Skip empty lines
            if line.tokens_ref().is_empty() {
                continue;
            }
            let mut tokens = line.tokens();
            let first_token = tokens.remove(0);
            match first_token.token_kind_ref() {
                TokenKind::Command(command) => match command {
                    Command::Alias => note.set_alias(first_token, tokens)?,
                    Command::Id => note.set_id(first_token, tokens)?,
                    Command::Docid => note.set_docid(first_token, tokens)?,
                    Command::Author => note.set_author(first_token, tokens)?,
                    Command::PaymentKind => note.set_payment_kind(first_token, tokens)?,
                    Command::Net => note.set_net(first_token, tokens)?,
                    Command::Gross => note.set_gross(first_token, tokens)?,
                    Command::Vat => note.set_vat(first_token, tokens)?,
                    Command::IssueDate => note.set_issuedate(first_token, tokens)?,
                    Command::CompletionDate => note.set_completiondate(first_token, tokens)?,
                    Command::DueDate => note.set_duedate(first_token, tokens)?,
                    Command::Transaction => note.set_transaction(first_token, tokens)?,
                    Command::Signature => (),
                    Command::Account => match is_account_file {
                        true => note.set_account(first_token, tokens)?,
                        false => {
                            return Err(error_msg(
                                &first_token,
                                "Account cannot be defined in a note file",
                            ))
                        }
                    },
                    Command::Unknown => return Err(error_msg(&first_token, "Unknown command")),
                },
                TokenKind::Text(_) => (), // Skip text lines
            }
        }
        note.check(is_account_file)?;
        Ok(note)
    }
    fn set_alias(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "Alias must have one parameter"));
        }
        self.alias = params.remove(0).token_kind().take_text_string();
        Ok(())
    }
    fn set_id(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "ID must have one parameter"));
        }
        self.id = params.remove(0).token_kind().take_text_string();
        Ok(())
    }
    fn set_docid(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "DOCID must have one parameter"));
        }
        self.docid = params.remove(0).token_kind().take_text_string();
        Ok(())
    }
    fn set_author(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() < 1 {
            return Err(error_msg(
                &first_token,
                "AUTHOR must have at least one parameter",
            ));
        }
        let value = params
            .into_iter()
            .map(|i| i.token_kind().take_text_string().unwrap())
            .collect::<Vec<String>>()
            .join(" ");
        self.author = Some(value);
        Ok(())
    }
    fn set_net(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "NET must have one parameter"));
        }
        self.net = match params.remove(0).token_kind().take_text_string() {
            Some(text) => match text.replace("_", "").parse::<i64>() {
                Ok(res) => res,
                Err(_) => return Err(error_msg(&first_token, "NET must be integer number")),
            },
            None => return Err(error_msg(&first_token, "NET must have one parameter")),
        };
        Ok(())
    }
    fn set_gross(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "GROSS must have one parameter"));
        }
        self.gross = match params.remove(0).token_kind().take_text_string() {
            Some(text) => match text.replace("_", "").parse::<i64>() {
                Ok(res) => res,
                Err(_) => return Err(error_msg(&first_token, "GROSS must be integer number")),
            },
            None => return Err(error_msg(&first_token, "GROSS must have one parameter")),
        };
        Ok(())
    }
    fn set_vat(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "VAT must have one parameter"));
        }
        self.vat = match params.remove(0).token_kind().take_text_string() {
            Some(text) => match text.replace("_", "").parse::<i64>() {
                Ok(res) => res,
                Err(_) => return Err(error_msg(&first_token, "VAT must be integer number")),
            },
            None => return Err(error_msg(&first_token, "VAT must have one parameter")),
        };
        Ok(())
    }
    fn set_payment_kind(
        &mut self,
        first_token: Token,
        mut params: Vec<Token>,
    ) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "AUTHOR must have one parameter"));
        }
        self.payment_kind = match params.remove(0).token_kind().take_text_string() {
            Some(text) => match text.as_str() {
                "cash" => Some(PaymentKind::Cash),
                "card" => Some(PaymentKind::Card),
                "transfer" => Some(PaymentKind::Transfer),
                _ => {
                    return Err(error_msg(
                        &first_token,
                        "PAYMENT_KIND value error. Must be: card, cash or transfer",
                    ))
                }
            },
            None => {
                return Err(error_msg(
                    &first_token,
                    "PAYMENT_KIND must have one parameter",
                ))
            }
        };
        Ok(())
    }
    fn set_issuedate(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "ISSUEDATE must have one parameter"));
        }
        let date_string = params.remove(0).token_kind().take_text_string().unwrap();
        let date = match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                return Err(error_msg(
                    &first_token,
                    "Date must have valid ISO format. YYYY-mm-dd",
                ))
            }
        };
        self.issue_date = Some(date);
        Ok(())
    }
    fn set_completiondate(
        &mut self,
        first_token: Token,
        mut params: Vec<Token>,
    ) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(
                &first_token,
                "COMPLETIONDATE must have one parameter",
            ));
        }
        let date_string = params.remove(0).token_kind().take_text_string().unwrap();
        let date = match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                return Err(error_msg(
                    &first_token,
                    "Date must have valid ISO format. YYYY-mm-dd",
                ))
            }
        };
        self.completion_date = Some(date);
        Ok(())
    }
    fn set_duedate(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() != 1 {
            return Err(error_msg(&first_token, "DUEDATE must have one parameter"));
        }
        let date_string = params.remove(0).token_kind().take_text_string().unwrap();
        let date = match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                return Err(error_msg(
                    &first_token,
                    "Date must have valid ISO format. YYYY-mm-dd",
                ))
            }
        };
        self.duedate = Some(date);
        Ok(())
    }
    fn set_transaction(
        &mut self,
        first_token: Token,
        mut params: Vec<Token>,
    ) -> Result<(), String> {
        if params.len() < 3 {
            return Err(error_msg(
                &first_token,
                "TRANSACTION must have minimum 3 parameters. Debit, Credit, Amount",
            ));
        }
        let mut transaction: Transaction = Transaction::default();
        transaction.debit = params
            .get(0)
            .unwrap()
            .token_kind_ref()
            .take_text_string_ref()
            .unwrap()
            .clone();
        transaction.credit = params
            .get(1)
            .unwrap()
            .token_kind_ref()
            .take_text_string_ref()
            .unwrap()
            .clone();
        transaction.amount = match params
            .get(2)
            .unwrap()
            .token_kind_ref()
            .take_text_string_ref()
            .unwrap()
            .replace("_", "")
            .parse::<i64>()
        {
            Ok(res) => res,
            Err(_) => {
                return Err(error_msg(
                    &first_token,
                    "TRANSACTION amount must be integer number",
                ))
            }
        };
        transaction.id = self.transactions.len() as i32 + 1;
        self.transactions.push(transaction);
        Ok(())
    }
    fn set_account(&mut self, first_token: Token, mut params: Vec<Token>) -> Result<(), String> {
        if params.len() < 2 {
            return Err(error_msg(
                &first_token,
                "ACCOUNT must have at least two parameter",
            ));
        }
        let account_name = params.remove(0).token_kind().take_text_string().unwrap();
        let account_value = params
            .into_iter()
            .map(|i| i.token_kind().take_text_string().unwrap())
            .collect::<Vec<String>>()
            .join(" ");
        let account = Account::new(account_name, account_value);
        self.accounts.push(account);
        Ok(())
    }
    fn check(&self, is_account_file: bool) -> Result<(), String> {
        if !is_account_file && self.id.is_none() {
            return Err(format!("MISSING ID at {}", self.path.to_string_lossy()));
        }
        Ok(())
    }
}

fn parse_single_string(tokens: &Vec<Token>) -> Result<String, String> {
    if tokens.len() > 2 {
        return Err(error_msg(
            tokens.last().unwrap(),
            "Too many parameters. Maximum 1.",
        ));
    }
    if tokens.len() > 2 {
        return Err(error_msg(
            tokens.last().unwrap(),
            "Few parameters. Need exactly 1.",
        ));
    }
    match tokens.get(1) {
        Some(t) => match t.token_kind_ref() {
            TokenKind::Command(_) => todo!(),
            TokenKind::Text(text) => todo!(),
        },
        None => unimplemented!(),
    }
}

fn error_msg(token: &Token, message: &str) -> String {
    format!(
        "Error row {} column {}\n{}",
        token.position().0,
        token.position().1,
        message
    )
}

#[derive(Debug)]
pub enum PaymentKind {
    Cash,
    Card,
    Transfer,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub id: i32,
    pub debit: String,
    pub credit: String,
    pub amount: i64,
}
