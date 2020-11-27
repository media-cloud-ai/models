use crate::common::{Identifier, ParameterType, Store};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "parameter")]
pub struct Parameter {
  /// Identifier of the parameter, used to identify it
  pub id: Identifier,
  /// Type of the data value for this parameter
  #[serde(flatten)]
  pub kind: ParameterType,
  // The Identifier of the store from which the parameter value can be resolved (as a credential)
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub store: Option<Store>,
}
