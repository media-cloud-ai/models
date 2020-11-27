use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "snake_case", rename = "type")]
pub enum StartParameterType {
  File,
  Choice,
  #[serde(rename = "string")]
  String,
  Number,
}
