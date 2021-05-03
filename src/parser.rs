use chrono::NaiveDate;
use uuid::Uuid;

trait Parser
where
  Self: Sized,
{
  fn parse(from: &str) -> Result<Self, String>;
}

#[derive(Debug, Clone)]
enum PreProcessToken {
  Comment(String),
  Text(String),
}

impl PartialEq for PreProcessToken {
  fn eq(&self, other: &Self) -> bool {
    match self {
      PreProcessToken::Comment(i) => match other {
        PreProcessToken::Comment(_i) => i == _i,
        PreProcessToken::Text(_) => false,
      },
      PreProcessToken::Text(i) => match other {
        PreProcessToken::Comment(_) => false,
        PreProcessToken::Text(_i) => i == _i,
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
        _ => PreProcessToken::Text(line.trim().to_string()),
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
            PreProcessToken::Text(_inner) => {
              res.push(token.take().unwrap());
              token.get_or_insert(token_line);
            }
          },
          PreProcessToken::Text(i) => match &token_line {
            // Change opened token stream
            PreProcessToken::Comment(_inner) => {
              res.push(token.take().unwrap());
              token.get_or_insert(token_line);
            }
            // Add rolling
            PreProcessToken::Text(inner) => {
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

enum Expression {
  DocComment(String),
  Mode(Mode),
  Account {
    id: String,
    name: String,
  },
  Transaction {
    debit: String,
    credit: String,
    event_id: Option<Uuid>,
    cdate: Option<NaiveDate>,
    amount: i32,
  },
  Reference {
    id: String,
    idate: Option<NaiveDate>,
    cdate: NaiveDate,
    ddate: Option<NaiveDate>,
  },
  Event {
    reference_id: String,
    name: String,
    idate: Option<NaiveDate>,
    cdate: Option<NaiveDate>,
    ddate: Option<NaiveDate>,
  },
  // Balance {
  //   id: String,
  //   name: String,
  // },
  // Profit {
  //   id: String,
  //   name: String,
  // },
}

enum Mode {
  Account,
  Balance,
  Profit,
  Transaction,
}

impl Parser for Mode {
  fn parse(from: &str) -> Result<Self, String> {
    match from {
      "account" => Ok(Self::Account),
      _ => Err(format!("Unknown mode: {}", from)),
    }
  }
}

pub fn parse_str(source: &str) -> () {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_preprocess() {
    let source = r#"
    
      # Hello bello
      # lorem ipsum dolorem;
      # set ami

      lorem ipsum dolorem

      #     ab

            line A

          #ab

      line B



      #ab

    "#;

    let result = pre_process(&source);
    let expected = vec![
      PreProcessToken::Comment("Hello bello\nlorem ipsum dolorem;\nset ami".to_string()),
      PreProcessToken::Text("lorem ipsum dolorem".to_string()),
      PreProcessToken::Comment("ab".to_string()),
      PreProcessToken::Text("line A".to_string()),
      PreProcessToken::Comment("ab".to_string()),
      PreProcessToken::Text("line B".to_string()),
      PreProcessToken::Comment("ab".to_string()),
    ];
    assert_eq!(result, expected);
  }
}
