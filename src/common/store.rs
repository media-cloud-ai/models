use regex::Regex;
use schemars::{
  gen::SchemaGenerator,
  schema::{InstanceType, Schema, SchemaObject, StringValidation},
  JsonSchema,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

static STORE_REGEX: &str = "^([A-Z0-9_]+)$";

lazy_static! {
  static ref REGEX: Regex = Regex::new(STORE_REGEX).unwrap();
}

#[derive(Debug)]
pub struct Store {
  store: String,
}

impl Store {
  pub fn new(store: String) -> Result<Self, String> {
    if !REGEX.is_match(&store) {
      return Err(format!(
        "{} does not match the store regex ({})",
        store, STORE_REGEX
      ));
    }
    Ok(Store { store })
  }
}

impl fmt::Display for Store {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.store)
  }
}

impl<'de> Deserialize<'de> for Store {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let store = String::deserialize(deserializer)?;
    Store::new(store).map_err(serde::de::Error::custom)
  }
}

impl Serialize for Store {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.store)
  }
}

impl JsonSchema for Store {
  fn schema_name() -> String {
    "store".to_owned()
  }

  fn json_schema(_: &mut SchemaGenerator) -> Schema {
    SchemaObject {
      instance_type: Some(InstanceType::String.into()),
      string: Some(Box::new(StringValidation {
        pattern: Some(STORE_REGEX.to_owned()),
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
