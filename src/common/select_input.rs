use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "select_input", rename_all = "snake_case")]
pub struct SelectInput {
  #[serde(default)]
  pub ends_with: Vec<String>,
}
