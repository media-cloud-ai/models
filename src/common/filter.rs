use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "filter", rename_all = "snake_case")]
pub struct Filter {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub ends_with: Vec<String>,
}
