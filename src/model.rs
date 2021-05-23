use crate::parser::Expression;

#[derive(Debug)]
pub struct Processed {
  accounts: Vec<()>,
  references: Vec<()>,
  events: Vec<()>,
  transactions: Vec<()>,
}

#[derive(Debug)]
pub struct Model {
  global: Option<()>,
  mode: Option<()>,
  accounts: Option<()>,
  reference: Option<()>,
  event: Option<()>,
}

impl Model {
  pub fn process(input: Vec<Expression>) -> Result<Processed, String> {
    todo!()
  }
}
