use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Kind {
    Note(NoteObj),
    Account(Account),
}

#[derive(Serialize, Deserialize, Debug)]
struct NoteObj {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ddate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    net: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gross: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    c: String,
    d: String,
    amount: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    name: String,
}

fn create_account(id: &str, name: &str) -> Account {
    Account {
        id: id.into(),
        name: name.into(),
    }
}

fn create_note() -> NoteObj {
    NoteObj {
        id: "2024-ab-0001".into(),
        cdate: Utc::now().date_naive().into(),
        ddate: Utc::now().date_naive().into(),
        idate: Utc::now().date_naive().into(),
        net: Some(100.0),
        vat: Some(27.0),
        gross: Some(127.0),
        comment: None,
        transactions: vec![
            Transaction {
                c: "1".into(),
                d: "2".into(),
                amount: 100.0,
                comment: Some("This is a sample comment".into()),
                cdate: None,
            },
            Transaction {
                c: "1".into(),
                d: "2".into(),
                amount: 27.0,
                comment: Some("This is a sample comment".into()),
                cdate: None,
            },
        ],
    }
}

fn write() {
    let n = create_note();
    let s = serde_yaml::to_string(&n).unwrap();
    let mut f = std::fs::File::create("demo_note.txt").unwrap();
    f.write_all(s.as_bytes()).unwrap();
}

fn write_list(to: &str) {
    let mut f = File::create(to).unwrap();
    let n = vec![
        Kind::Account(create_account("1", "Assets")),
        Kind::Account(create_account("2", "Demo")),
        Kind::Account(create_account("3", "Wowow")),
    ];
    serde_yaml::to_writer(f, &n).unwrap();
}

fn read() -> NoteObj {
    let mut f = File::open("demo/d.yaml").unwrap();
    let mut c = String::new();
    f.read_to_string(&mut c).unwrap();
    serde_yaml::from_str(&c).unwrap()
}

fn main() {
    write_list("demo/list.yaml");
}
