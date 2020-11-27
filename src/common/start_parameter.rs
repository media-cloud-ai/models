use super::{Icon, InterfaceParameterType, ListItem, StartParameterType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields, rename = "start_parameter")]
pub struct StartParameter {
  /// A list of acceptable file type
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub accept: Option<String>,
  /// Identifier of the parameter
  pub id: String,
  /// Label of the parameter
  pub label: String,
  /// Type of the data value for this parameter
  #[serde(rename = "type")]
  pub kind: StartParameterType,
  /// Default value of the parameter
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub default: Option<InterfaceParameterType>,
  /// Value of the parameter
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub value: Option<InterfaceParameterType>,
  /// Set the parameter required
  #[serde(default, skip_serializing_if = "crate::is_false")]
  pub required: bool,
  /// Set the icon for this parameter
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub icon: Option<Icon>,
  /// Step for float input
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub step: Option<f32>,
  /// Overwrite de default workers work directory
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub work_dir: Option<String>,
  /// List of choice identifier and value
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub items: Vec<ListItem>,
}
