use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", rename = "mode")]
pub enum StepMode {
  OneForMany,
  OneForOne,
  Notification,
}

impl StepMode {
  pub fn is_default(v: &StepMode) -> bool {
    *v == StepMode::default()
  }
}

impl Default for StepMode {
  fn default() -> StepMode {
    StepMode::OneForOne
  }
}
