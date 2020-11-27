use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "rights", rename_all = "snake_case")]
pub struct Right {
  pub action: RightAction,
  pub groups: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "action", rename_all = "snake_case")]
pub enum RightAction {
  Abort,
  Create,
  Delete,
  Retry,
  Update,
  View,
}
