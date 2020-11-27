use super::{Icon, Parameter, StepMode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Definition of a step
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "step")]
pub struct Step {
  /// The icon for this step
  pub icon: Icon,
  /// Unique identifier of this step in a workflow
  pub id: u32,
  /// Displayed name of this step
  pub label: String,
  /// Queue name for this step, it select the worker
  pub name: String,
  /// List of all parameters for this step
  pub parameters: Vec<Parameter>,
  /// Mode for this step
  #[serde(default, skip_serializing_if = "StepMode::is_default")]
  pub mode: StepMode,
  /// Name of the variable which contain data ranges to process splitted media by part
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub multiple_jobs: Option<String>,
  /// Expression which requires to return a boolean. On false condition, the step will be skipped.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub condition: Option<String>,
  /// Skip generation of a destination path parameter, it requires to add it in parameters
  #[serde(default, skip_serializing_if = "crate::is_false")]
  pub skip_destination_path: bool,
  /// Reference(s) of parent steps
  ///
  /// It is used to generate the input paths list from all parents destination paths
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub parent_ids: Vec<u32>,
  /// Reference(s) of required steps to start the process of that step
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub required_to_start: Vec<u32>,
  /// Overwrite the default workdir for this step
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub work_dir: Option<String>,
}
