use std::{
  ops::Not,
  path::{Path, PathBuf},
  str::Bytes,
};

#[derive(Default, Debug)]
pub struct NoteRaw {
  file_path: PathBuf,
  lines: Vec<Line>,
  is_valid: bool,
  is_signed: bool,
}

impl NoteRaw {
  fn add_line(&mut self, new_line: Line) {
    self.lines.push(new_line);
  }
  pub fn from_file(file_path: &Path) -> Result<Self, String> {
    let contents =
      std::fs::read_to_string(&file_path).expect("Something went wrong reading the file");

    let mut note_raw: NoteRaw = NoteRaw::default();
    note_raw.file_path = file_path.to_owned();
    for line in contents.lines().enumerate() {
      let new_line = Line::from_raw_line(LineRaw::new(line.0, line.1.to_string()));
      note_raw.add_line(new_line);
    }
    Ok(note_raw)
  }
  pub fn is_signed(&self) -> bool {
    self.is_signed
  }
  pub fn raw_bytes(&self) -> Vec<u8> {
    let a = self.lines.iter().map(|i| i.raw()).collect::<Vec<&str>>();
    a.join("\n").into_bytes()
  }
  pub fn file_path(&self) -> &Path {
    &self.file_path
  }
  pub fn lines_ref(&self) -> &Vec<Line> {
    &self.lines
  }
  pub fn lines(self) -> Vec<Line> {
    self.lines
  }
}

#[derive(Debug)]
struct LineRaw {
  line_number: usize,
  raw: String,
}

impl LineRaw {
  fn new(line_number: usize, raw: String) -> Self {
    LineRaw { line_number, raw }
  }
}

#[derive(Debug)]
pub struct Line {
  raw: String,
  tokens: Vec<Token>,
}

impl Line {
  fn from_raw_line(raw_line: LineRaw) -> Self {
    Self {
      raw: raw_line.raw.clone(),
      tokens: tokenize_line(raw_line),
    }
  }
  pub fn raw(&self) -> &str {
    &self.raw
  }
  pub fn tokens_ref(&self) -> &Vec<Token> {
    &self.tokens
  }
  pub fn tokens(self) -> Vec<Token> {
    self.tokens
  }
}

fn tokenize_line(raw_line: LineRaw) -> Vec<Token> {
  let mut tokens: Vec<Token> = vec![];
  let mut temp_token: Option<Token> = None;
  let mut token_is_inner = false;

  for (char_pos, ch) in raw_line.raw.chars().enumerate() {
    match ch {
      ' ' => match token_is_inner {
        true => temp_token.as_mut().unwrap().append_char(ch),
        false => {
          let t = temp_token.take();
          if let Some(t) = t {
            tokens.push(t);
          }
        }
      },
      '"' => match token_is_inner {
        false => {
          token_is_inner = true;
          temp_token = Some(Token::new(
            (raw_line.line_number + 1, char_pos + 1),
            TokenKind::Text(String::new()),
          ));
          continue;
        }
        true => {
          token_is_inner = false;
          continue;
        }
      },
      x => match token_is_inner {
        false => match &mut temp_token {
          Some(t) => {
            t.append_char(x);
          }
          None => {
            let mut s = String::new();
            s.push(x);
            temp_token = Some(Token::new(
              (raw_line.line_number + 1, char_pos + 1),
              TokenKind::Text(s),
            ));
          }
        },
        true => temp_token.as_mut().unwrap().append_char(x),
      },
    }
  }

  let t = temp_token.take();
  if let Some(t) = t {
    tokens.push(t);
  }

  if let Some(first_token) = tokens.get_mut(0) {
    match &mut first_token.token_kind {
      TokenKind::Command(_) => (),
      TokenKind::Text(t) => {
        let c = Command::parse(&t);
        match c {
          Command::Unknown => (),
          x => first_token.token_kind = TokenKind::Command(x),
        }
      }
    }
  }

  tokens
}

#[derive(Debug)]
pub struct Token {
  position: (usize, usize),
  token_kind: TokenKind,
}

impl Token {
  fn new(position: (usize, usize), token_kind: TokenKind) -> Self {
    Self {
      position,
      token_kind,
    }
  }
  fn append_char(&mut self, ch: char) {
    match &mut self.token_kind {
      TokenKind::Command(_) => (),
      TokenKind::Text(t) => t.push(ch),
    }
  }
  pub fn position(&self) -> (usize, usize) {
    self.position
  }
  pub fn token_kind_ref(&self) -> &TokenKind {
    &self.token_kind
  }
  pub fn token_kind(self) -> TokenKind {
    self.token_kind
  }
}

#[derive(Debug)]
pub enum TokenKind {
  Command(Command),
  Text(String),
}

impl TokenKind {
  pub fn take_text_string(self) -> Option<String> {
    match self {
      TokenKind::Command(_) => None,
      TokenKind::Text(text) => Some(text),
    }
  }
  pub fn take_text_string_ref(&self) -> Option<&String> {
    match self {
      TokenKind::Command(_) => None,
      TokenKind::Text(text) => Some(text),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Command {
  Alias,
  Id,
  Docid,
  Author,
  PaymentKind,
  Net,
  Gross,
  Vat,
  IssueDate,
  CompletionDate,
  DueDate,
  Transaction,
  Signature,
  Account,
  Unknown,
}

impl Command {
  fn parse(f: &str) -> Self {
    match f {
      "ALIAS" => Self::Alias,
      "ID" => Self::Id,
      "DOCID" => Self::Docid,
      "AUTHOR" => Self::Author,
      "PAYMENT_KIND" => Self::PaymentKind,
      "NET" => Self::Net,
      "GROSS" => Self::Gross,
      "VAT" => Self::Vat,
      "ISSUE_DATE" => Self::IssueDate,
      "COMPLETION_DATE" => Self::CompletionDate,
      "DUEDATE" => Self::DueDate,
      "TRANSACTION" | ">" => Self::Transaction,
      "SIGNATURE" | "!" => Self::Signature,
      "ACCOUNT" | "%" => Self::Account,
      _ => Self::Unknown,
    }
  }
}

fn parse(note_string: &str) -> NoteRaw {
  let mut note_raw: NoteRaw = NoteRaw::default();
  for line in note_string.lines().enumerate() {
    let new_line = Line::from_raw_line(LineRaw::new(line.0, line.1.to_string()));
    note_raw.add_line(new_line);
  }
  note_raw
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn test_preprocess() {
    let source = r#"---
a "a b c" c d "a b c d e"
Nyomdai kellékek
vásárlása
---

ALIAS         SZG-2022-10079
ID            12a3ef
DOCID         447af
DATE          2022-05-08
AUTHOR        Peter Mezei
PAYMENT_KIND  transfer
NET           11.811
GROSS         15.000
VAT           3.189
ISSUEDATE     2022-04-01
COMPDATE      2022-04-01
DUEDATE       2022-04-31

Nettó szállítóra könyvelve
egyből költségként
> T5    K454/agroker 11_811

Áfa elszámolása
Ahogy, hogy rendben legyen úgy számolunk, hogy ...
Az alábbi képlettel:
  x = 2 * x
> T466  K454/agroker 3_189

! 3bc9cccd8fb90b9e836b9b6a7ef1c62ds

PAYMENT_KIND  cash

! 18db35eccfa2d4e3448c523985518c0b
    "#;

    let parsed = parse(&source);
    assert_eq!(true, true);
    println!("{:?}", parsed);
  }

  #[test]
  fn test2() {
    let source = r#"
ALIAS 2022/01/000009
ID 1
DOCID 2
NET 10000
VAT 2700
GROSS 12700
COMPLETION_DATE 2022-01-01
DUEDATE 2022-01-31

% 161 Beruházás
% 3841 "Peti bank"

> 161 3841 12000

    "#;

    let parsed = parse(&source);
    assert_eq!(true, true);
    println!("{:?}", parsed);
  }
}
