use super::{Filter, MediaSegment, Requirement, SelectInput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", rename = "type")]
pub enum ParameterType {
  ArrayOfMediaSegments {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    default: Vec<MediaSegment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    value: Vec<MediaSegment>,
  },
  ArrayOfStrings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    default: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    value: Vec<String>,
  },
  ArrayOfTemplates {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    default: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    value: Vec<String>,
  },
  Boolean {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<bool>,
  },
  Extended {
    default: serde_json::Value,
    value: serde_json::Value,
  },
  Filter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<Filter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<Filter>,
  },
  Integer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
  },
  Requirements {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<Requirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<Requirement>,
  },
  SelectInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<SelectInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<SelectInput>,
  },
  String {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<String>,
  },
  Template {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<String>,
  },
}
