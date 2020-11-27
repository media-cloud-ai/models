use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "snake_case", rename = "interface_parameter_type")]
pub enum InterfaceParameterType {
  Integer,
  Number,
  String,
  Array,
}
