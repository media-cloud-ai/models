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
pub struct Identifier {
  content: String,
}

impl Identifier {
  pub fn new(content: String) -> Result<Self, String> {
    if !REGEX.is_match(&content) {
      return Err(format!(
        "{} does not match the identifier regex ({})",
        content, IDENTIFIER_REGEX
      ));
    }
    Ok(Identifier { content })
  }
}

impl fmt::Display for Identifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.content)
  }
}

impl<'de> Deserialize<'de> for Identifier {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let identifier = String::deserialize(deserializer)?;
    Identifier::new(identifier).map_err(serde::de::Error::custom)
  }
}

impl Serialize for Identifier {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.content)
  }
}

impl JsonSchema for Identifier {
  fn schema_name() -> String {
    "identifier".to_owned()
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
