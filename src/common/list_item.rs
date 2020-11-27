use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "list_item")]
pub struct ListItem {
  /// Value returned on selected item
  id: String,
  /// Displayed value on user interface
  label: String,
}
