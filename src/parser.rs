use chrono::NaiveDate;
use uuid::Uuid;

trait Parser
where
  Self: Sized,
{
  fn parse(from: &Vec<(String, String)>) -> Result<Self, String>;
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

pub enum Expression {
  DocComment(CommentExp),
  Mode(ModeExp),
  Account(AccountExp),
  Transaction(TransactionExp),
  Reference(ReferenceExp),
  Event(EventExp),
}

pub enum ModeExp {
  Account,
  Balance,
  Profit,
  Transaction,
}

impl Parser for ModeExp {
  fn parse(params: &Vec<(String, String)>) -> Result<Self, String> {
    let first = &params[0];
    match first.0.as_str() {
      "NAME" => match first.1.as_str() {
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

pub struct CommentExp(String);

pub struct AccountExp {
  id: String,
  name: String,
}

pub struct TransactionExp {
  debit: String,
  credit: String,
  event_id: Option<Uuid>,
  cdate: Option<NaiveDate>,
  amount: i32,
}

pub struct ReferenceExp {
  id: String,
  idate: Option<NaiveDate>,
  cdate: NaiveDate,
  ddate: Option<NaiveDate>,
}

pub struct EventExp {
  reference_id: String,
  name: String,
  idate: Option<NaiveDate>,
  cdate: Option<NaiveDate>,
  ddate: Option<NaiveDate>,
}

fn parse_exp_candidate(candidate: &str) -> Result<Expression, String> {
  todo!()
}

fn parse_text_block(text_block: &str) -> Result<Vec<Expression>, String> {
  let mut res = Vec::new();
  let exp_candidates = text_block.split(";").collect::<Vec<_>>();
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

#[cfg(test)]
mod tests {
  use super::*;

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
    let expr = r#"TRANSACTION name Lorem debit 161 credit 3841 amount 4500"#;
    let res = ExpressionCandidate::from_str(&expr);
    println!("{:?}", &res);
    assert_eq!(res.is_ok(), true);

    let expr_quoted =
      r#"TRANSACTION name "Lorem ipsum dolorem set ami" debit 161 credit 3841 amount 4500"#;
    let res = ExpressionCandidate::from_str(&expr_quoted);
    println!("{:?}", &res);
    assert_eq!(res.is_ok(), true);
  }

  #[test]
  fn test_param_parse() {
    let token_stream = "NAME lorem AGE 32 IMPORTANT";
  }
}
