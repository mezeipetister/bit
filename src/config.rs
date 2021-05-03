use serde::Deserialize;

/// BIT Config
#[derive(Deserialize, Debug)]
pub struct Config {
  pub name: String,
  pub description: Option<String>,
  pub year: String,
  pub currency: String,
  pub dependencies: Dependencies,
}

#[derive(Deserialize, Debug)]
pub struct Dependencies {
  pub accounts: String,
  pub balance_sheet: String,
  pub profit_loss: String,
  pub logs: String,
}
