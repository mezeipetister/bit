use std::{fs::File, io::Read, path::Path};

use chrono::NaiveDate;

trait Parser
where
  Self: Sized,
{
  fn parse_params(from: &Vec<(String, String)>) -> Result<Self, String>;
}

#[derive(Debug, Clone)]
enum PreProcessToken {
  Comment(String),
  TextBlock(String),
}

impl PartialEq for PreProcessToken {
  fn eq(&self, other: &Self) -> bool {
    match self {
      PreProcessToken::Comment(i) => match other {
        PreProcessToken::Comment(_i) => i == _i,
        PreProcessToken::TextBlock(_) => false,
      },
      PreProcessToken::TextBlock(i) => match other {
        PreProcessToken::Comment(_) => false,
        PreProcessToken::TextBlock(_i) => i == _i,
      },
    }
  }
}

fn pre_process(input: &str) -> Vec<PreProcessToken> {
  let mut res = Vec::new();
  let mut token: Option<PreProcessToken> = None;
  for line in input.lines() {
    // Skip empty lines
    if line.trim().is_empty() {
      continue;
    }
    // If there is at least one char
    if let Some(fc) = line.trim_start().chars().nth(0) {
      let token_line = match fc {
        // Doc comment
        '#' => PreProcessToken::Comment(line.trim().trim_matches('#').trim().to_string()),
        // Else
        _ => PreProcessToken::TextBlock(
          line
            .trim()
            .split("//")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .trim()
            .to_string(),
        ),
      };
      match &mut token {
        Some(tkn) => match tkn {
          PreProcessToken::Comment(i) => match &token_line {
            // Add rolling
            PreProcessToken::Comment(inner) => {
              i.push_str("\n");
              i.push_str(inner);
            }
            // Change opened token stream
            PreProcessToken::TextBlock(_inner) => {
              res.push(token.take().unwrap());
              token.get_or_insert(token_line);
            }
          },
          PreProcessToken::TextBlock(i) => match &token_line {
            // Change opened token stream
            PreProcessToken::Comment(_inner) => {
              res.push(token.take().unwrap());
              token.get_or_insert(token_line);
            }
            // Add rolling
            PreProcessToken::TextBlock(inner) => {
              i.push_str("\n");
              i.push_str(inner);
            }
          },
        },
        None => token = Some(token_line),
      }
    }
  }
  res.push(token.take().unwrap());
  res
}

#[derive(Debug)]
struct ExpressionCandidate {
  command: String,
  parameters: Vec<(String, String)>,
}

impl ExpressionCandidate {
  fn from_str(single_expression: &str) -> Result<ExpressionCandidate, String> {
    // Preprocess token stream
    let mut token_stream = single_expression
      .replace(";", "")
      .trim()
      .split_whitespace()
      .collect::<Vec<_>>()
      .into_iter()
      .map(|i| i.trim().to_string())
      .collect::<Vec<_>>();

    if token_stream.len() < 1 {
      return Err("Empty expression. Impossible error!".to_string());
    }

    let command = token_stream.remove(0);

    // Restore " string literals
    let mut _token_stream: Vec<String> = Vec::new();

    let mut append_mode = false;

    for token in token_stream {
      let is_opening = if let Some(first_char) = token.chars().collect::<Vec<_>>().first() {
        if *first_char == '"' {
          true
        } else {
          false
        }
      } else {
        false
      };

      let is_closing = if let Some(last_char) = token.chars().collect::<Vec<_>>().last() {
        if *last_char == '"' {
          true
        } else {
          false
        }
      } else {
        false
      };

      match append_mode {
        true => match _token_stream.len() {
          // Concat token to the last one
          x if x > 0 => {
            if let Some(last) = _token_stream.last_mut() {
              last.push_str(" ");
              last.push_str(&token);
            }
          }
          // First item
          _ => _token_stream.push(token),
        },
        false => _token_stream.push(token),
      }

      if is_opening {
        match append_mode {
          true => return Err("Syntax error! Quotation in quotation".to_string()),
          false => append_mode = true,
        }
      }

      if is_closing {
        append_mode = false;
      }
    }

    let mut parameters: Vec<(String, String)> = Vec::new();

    for p in _token_stream.chunks(2) {
      if p.len() != 2 {
        return Err(format!("No value for parameter {}", p[0]));
      }
      parameters.push((p[0].to_string(), p[1].to_string()));
    }

    Ok(ExpressionCandidate {
      command,
      parameters,
    })
  }
}

#[derive(Debug)]
pub enum Expression {
  DocComment(CommentExp),
  Mode(ModeExp),
  Account(AccountExp),
  Transaction(TransactionExp),
  Reference(ReferenceExp),
  Event(EventExp),
}

#[derive(Debug)]
pub enum ModeExp {
  Account,
  Balance,
  Profit,
  Transaction,
}

impl Parser for ModeExp {
  fn parse_params(params: &Vec<(String, String)>) -> Result<Self, String> {
    let first = &params[0];
    match first.0.as_str() {
      "set" | "SET" => match first.1.as_str() {
        "account" => Ok(ModeExp::Account),
        "balance" => Ok(ModeExp::Balance),
        "profit" => Ok(ModeExp::Profit),
        "transaction" => Ok(ModeExp::Transaction),
        _ => Err("Unknown mode".to_string()),
      },
      _ => Err("Unknown parameter for MODE".to_string()),
    }
  }
}

#[derive(Debug)]
pub struct CommentExp(String);

#[derive(Debug)]
pub struct AccountExp {
  pub id: String,
  pub name: String,
}

impl Parser for AccountExp {
  fn parse_params(from: &Vec<(String, String)>) -> Result<Self, String> {
    let mut id: Option<String> = None;
    let mut name: Option<String> = None;
    for row in from {
      match row.0.as_str() {
        "id" | "ID" => id = Some(row.1.to_string()),
        "name" | "NAME" => name = Some(row.1.to_string()),
        _ => return Err("Unknown parameter".to_string()),
      }
    }
    Ok(Self {
      id: id.ok_or("No ID given".to_string())?,
      name: name.ok_or("No NAME given".to_string())?,
    })
  }
}

#[derive(Debug)]
pub struct TransactionExp {
  pub debit: String,
  pub credit: String,
  pub event_id: Option<String>,
  pub cdate: Option<NaiveDate>,
  pub amount: i64,
}

impl Parser for TransactionExp {
  fn parse_params(from: &Vec<(String, String)>) -> Result<Self, String> {
    let mut debit: Option<String> = None;
    let mut credit: Option<String> = None;
    let mut event_id: Option<String> = None;
    let mut cdate: Option<NaiveDate> = None;
    let mut amount: Option<i64> = None;
    for row in from {
      match row.0.as_str() {
        "debit" | "DEBIT" | "d" | "D" => debit = Some(row.1.to_string()),
        "credit" | "CREDIT" | "c" | "C" => credit = Some(row.1.to_string()),
        "event_id" | "EVENT_ID" => {
          event_id = Some(
            row
              .1
              .parse::<String>()
              .map_err(|_| "Event ID must be a string")?,
          )
        }
        "cdate" | "CDATE" => {
          cdate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong date format")?,
          )
        }
        "amount" | "AMOUNT" | "a" | "A" => {
          amount = Some(
            row
              .1
              .replace("_", "")
              .parse::<i64>()
              .map_err(|_| "Amount must be integer number")?,
          )
        }
        _ => return Err("Unknown parameter".to_string()),
      }
    }
    Ok(Self {
      debit: debit.ok_or("No debit given")?,
      credit: credit.ok_or("No credit given")?,
      event_id,
      cdate,
      amount: amount.ok_or("No amount given")?,
    })
  }
}

#[derive(Debug)]
pub struct ReferenceExp {
  pub id: String,
  pub name: Option<String>,
  pub idate: Option<NaiveDate>,
  pub cdate: NaiveDate,
  pub ddate: Option<NaiveDate>,
}

impl Parser for ReferenceExp {
  fn parse_params(from: &Vec<(String, String)>) -> Result<Self, String> {
    let mut id: Option<String> = None;
    let mut name: Option<String> = None;
    let mut idate: Option<NaiveDate> = None;
    let mut cdate: Option<NaiveDate> = None;
    let mut ddate: Option<NaiveDate> = None;
    for row in from {
      match row.0.as_str() {
        "id" | "ID" => id = Some(row.1.to_string()),
        "name" | "NAME" => name = Some(row.1.to_string()),
        "idate" | "IDATE" => {
          idate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong IDATE date format")?,
          )
        }
        "cdate" | "CDATE" => {
          cdate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong CDATE date format")?,
          )
        }
        "ddate" | "DDATE" => {
          ddate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong DDATE date format")?,
          )
        }
        _ => return Err("Unknown parameter".to_string()),
      }
    }
    Ok(Self {
      id: id.ok_or("Missing reference ID")?,
      name,
      idate,
      cdate: cdate.ok_or("Missing CDATE")?,
      ddate,
    })
  }
}

#[derive(Debug)]
pub struct EventExp {
  pub id: String,
  pub reference_id: String,
  pub name: Option<String>,
  pub idate: Option<NaiveDate>,
  pub cdate: Option<NaiveDate>,
  pub ddate: Option<NaiveDate>,
}

impl Parser for EventExp {
  fn parse_params(from: &Vec<(String, String)>) -> Result<Self, String> {
    let mut id = None;
    let mut reference_id: Option<String> = None;
    let mut name: Option<String> = None;
    let mut idate: Option<NaiveDate> = None;
    let mut cdate: Option<NaiveDate> = None;
    let mut ddate: Option<NaiveDate> = None;
    for row in from {
      match row.0.as_str() {
        "id" | "ID" => id = Some(row.1.to_string()),
        "reference_id" | "REFERENCE_ID" | "refid" => reference_id = Some(row.1.to_string()),
        "name" | "NAME" => name = Some(row.1.to_string()),
        "idate" | "IDATE" => {
          idate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong IDATE date format")?,
          )
        }
        "cdate" | "CDATE" => {
          cdate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong CDATE date format")?,
          )
        }
        "ddate" | "DDATE" => {
          ddate = Some(
            row
              .1
              .parse::<NaiveDate>()
              .map_err(|_| "Wrong DDATE date format")?,
          )
        }
        _ => return Err("Unknown parameter".to_string()),
      }
    }
    Ok(Self {
      id: id.ok_or("Missing event ID")?,
      reference_id: reference_id.ok_or("Missing reference ID")?,
      name,
      idate,
      cdate,
      ddate,
    })
  }
}

fn parse_exp_candidate(candidate: &str) -> Result<Expression, String> {
  let candidate = ExpressionCandidate::from_str(candidate)?;
  match candidate.command.as_str() {
    "mode" | "MODE" => Ok(Expression::Mode(ModeExp::parse_params(
      &candidate.parameters,
    )?)),
    "reference" | "REFERENCE" | "ref" => Ok(Expression::Reference(ReferenceExp::parse_params(
      &candidate.parameters,
    )?)),
    "event" | "EVENT" => Ok(Expression::Event(EventExp::parse_params(
      &candidate.parameters,
    )?)),
    "transaction" | "TRANSACTION" | "tr" => Ok(Expression::Transaction(
      TransactionExp::parse_params(&candidate.parameters)?,
    )),
    "account" | "ACCOUNT" => Ok(Expression::Account(AccountExp::parse_params(
      &candidate.parameters,
    )?)),
    _ => Err(format!("Unknown command: {}", candidate.command)),
  }
}

fn parse_text_block(text_block: &str) -> Result<Vec<Expression>, String> {
  let mut res = Vec::new();
  let exp_candidates = text_block
    .split(";")
    .filter(|i| !i.is_empty())
    .collect::<Vec<_>>();
  for ec in exp_candidates {
    res.push(parse_exp_candidate(ec)?);
  }
  Ok(res)
}

fn parse_expr(pre_tokens: Vec<PreProcessToken>) -> Result<Vec<Expression>, String> {
  let mut res = Vec::new();
  for token in pre_tokens {
    let mut expressions = match token {
      PreProcessToken::Comment(comment_string) => {
        vec![Expression::DocComment(CommentExp(comment_string))]
      }
      PreProcessToken::TextBlock(text_block) => parse_text_block(&text_block)?,
    };
    res.append(&mut expressions);
  }
  Ok(res)
}

/// Public parse interface
/// to generate expression list from
/// a given str
pub fn parse(input: &str) -> Result<Vec<Expression>, String> {
  let pre_tokens = pre_process(input);
  parse_expr(pre_tokens)
}
pub fn parse_file(file: &Path) -> Result<Vec<Expression>, String> {
  let mut f = File::open(&file).map_err(|_| "Error while opening file".to_string())?;
  let mut content = String::new();
  f.read_to_string(&mut content)
    .map_err(|_| "Error while reading file".to_string())?;
  parse(&content)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn test_preprocess() {
    let source = r#"
    
      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      lorem ipsum dolorem    // line comment B

      #     ab

            line A // line comment A "Hello"

          #ab

      line B // line comment C



      #ab

    "#;

    let result = pre_process(&source);
    let expected = vec![
      PreProcessToken::Comment(
        "Hello bello\n===========\nlorem ipsum dolorem;\nset ami".to_string(),
      ),
      PreProcessToken::TextBlock("lorem ipsum dolorem".to_string()),
      PreProcessToken::Comment("ab".to_string()),
      PreProcessToken::TextBlock("line A".to_string()),
      PreProcessToken::Comment("ab".to_string()),
      PreProcessToken::TextBlock("line B".to_string()),
      PreProcessToken::Comment("ab".to_string()),
    ];
    assert_eq!(result, expected);
  }

  #[test]
  fn test_expression_candidate() {
    let expr = r#"TRANSACTION name Lorem debit 161 credit 3841 amount 4500;"#;
    let res = ExpressionCandidate::from_str(&expr);
    println!("{:?}", &res);
    assert_eq!(res.is_ok(), true);

    let expr_quoted =
      r#"TRANSACTION name "Lorem ipsum dolorem set ami" debit 161 credit 3841 amount 4500; "#;
    let res = ExpressionCandidate::from_str(&expr_quoted);
    println!("{:?}", &res);
    assert_eq!(res.is_ok(), true);
  }

  #[test]
  fn test_param_parse() {
    let token_stream = "NAME lorem AGE 32 IMPORTANT";
  }

  #[test]
  fn test_exp_mode() {
    let params = vec![("SET".to_string(), "account".to_string())];
    assert_eq!(ModeExp::parse_params(&params).is_ok(), true);
  }

  #[test]
  fn test_exp_account() {
    let params = vec![
      ("ID".to_string(), "161".to_string()),
      ("NAME".to_string(), "lorem ipsum".to_string()),
    ];
    assert_eq!(AccountExp::parse_params(&params).is_ok(), true);
  }

  #[test]
  fn test_exp_transaction() {
    let params = vec![
      ("debit".to_string(), "161".to_string()),
      ("credit".to_string(), "38".to_string()),
      ("amount".to_string(), "40000".to_string()),
      ("amount".to_string(), "40_000".to_string()),
    ];
    let res = TransactionExp::parse_params(&params);
    assert_eq!(res.is_ok(), true);
  }

  #[test]
  fn test_exp_reference() {
    let params = vec![
      ("id".to_string(), "lorem ipsum dolorem".to_string()),
      ("cdate".to_string(), "2021-04-04".to_string()),
    ];
    let res = ReferenceExp::parse_params(&params);
    assert_eq!(res.is_ok(), true);
  }

  #[test]
  fn test_exp_event() {
    let params = vec![
      ("id".to_string(), "e1".to_string()),
      (
        "reference_id".to_string(),
        "lorem ipsum dolorem".to_string(),
      ),
      ("cdate".to_string(), "2021-04-04".to_string()),
    ];
    let res = EventExp::parse_params(&params);
    assert_eq!(res.is_ok(), true);
  }

  #[test]
  fn test_parse_complete() {
    let source = r#"
    
      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";


    "#;
    let res = parse(source);
    println!("{:?}", &res);
  }

  #[bench]
  fn bench_add_two(b: &mut Bencher) {
    let source = r#"
    
      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT id e1 reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000   // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";

      # Hello bello
      # ===========
      # lorem ipsum dolorem;
      # set ami

      # set mode
      MODE set account;

      reference
        id demo_ref_id
        name "Demo reference"
        cdate 2021-01-01;

      REFERENCE
        ID demo_ref_id
        NAME "Demo reference"
        CDATE 2021-01-01;

      EVENT reference_id demo_event_id name demo_event;

      # Demo transaction
      transaction
        debit 161       // Beruházás számla
        credit 3811     // Pénztár
        amount 34_000;  // Könyvelt nettó összeg

      # Demo account
      ACCOUNT
        ID 161
        NAME "Beruházás számla";


    "#;
    b.iter(|| {
      let a = parse(&source);
      a.is_ok();
    });
  }
}
