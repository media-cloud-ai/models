use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "requirement", rename_all = "snake_case")]
pub struct Requirement {
  #[serde(default)]
  pub paths: Vec<String>,
}
