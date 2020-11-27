use regex::Regex;
use schemars::{
  gen::SchemaGenerator,
  schema::{InstanceType, Schema, SchemaObject, StringValidation},
  JsonSchema,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

static IDENTIFIER_REGEX: &str = "^([a-z_]+)$";

lazy_static! {
  static ref REGEX: Regex = Regex::new(IDENTIFIER_REGEX).unwrap();
}

#[derive(Debug)]
pub struct Icon {
  icon: String,
}

impl Icon {
  pub fn new(icon: String) -> Result<Self, String> {
    if !REGEX.is_match(&icon) {
      return Err(format!(
        "{} does not match the identifier regex ({})",
        icon, IDENTIFIER_REGEX
      ));
    }
    Ok(Icon { icon })
  }
}

impl fmt::Display for Icon {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.icon)
  }
}

impl<'de> Deserialize<'de> for Icon {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let icon = String::deserialize(deserializer)?;
    Icon::new(icon).map_err(serde::de::Error::custom)
  }
}

impl Serialize for Icon {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.icon)
  }
}

impl JsonSchema for Icon {
  fn schema_name() -> String {
    "icon".to_owned()
  }

  fn json_schema(_: &mut SchemaGenerator) -> Schema {
    SchemaObject {
      instance_type: Some(InstanceType::String.into()),
      string: Some(Box::new(StringValidation {
        pattern: Some(IDENTIFIER_REGEX.to_owned()),
        ..Default::default()
      })),
      ..Default::default()
    }
    .into()
  }

  fn is_referenceable() -> bool {
    false
  }
}
