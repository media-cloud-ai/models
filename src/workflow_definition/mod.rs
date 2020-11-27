//! Workflow Definition
use crate::common::{Icon, Identifier, Parameter, Right, StartParameter, Step};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(tag = "schema_version")]
/// # Schema of Workflow Definition for Media Cloud AI
pub enum WorkflowDefinition {
  #[serde(rename = "1.8")]
  Version1_8(Version1_8),
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Version1_8 {
  /// The icon used with the label. One of [theses](https://material.io/resources/icons/) icons.
  pub icon: Icon,
  /// The Identifier of the workflow, used to reference it
  pub identifier: Identifier,
  /// The label of the workflow, used as displayed name
  pub label: String,
  /// Storage of dynamic parameters during process of the workflow instance
  pub parameters: Vec<Parameter>,
  /// External reference identifier for this workflow, can be UUID, string etc.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reference: Option<String>,
  /// Definition of available parameters to start the workflow definition
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub start_parameters: Vec<StartParameter>,
  /// Mentions if it defines a live workflow
  #[serde(default, skip_serializing_if = "crate::is_false")]
  pub is_live: bool,
  /// Defines rights for this definition
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub rights: Vec<Right>,
  /// Steps to represent the complete process
  pub steps: Vec<Step>,
  /// List of tags to classify the worklow
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tags: Vec<String>,
  /// Major version of this Workflow definition
  pub version_major: u32,
  /// Minor version of this Workflow definition
  pub version_minor: u32,
  /// Micro version of this Workflow definition
  pub version_micro: u32,
}
