#![feature(result_flattening)]

extern crate mcai_models;

use mcai_models::workflow_definition::WorkflowDefinition;
use schemars::schema_for;
use std::{env, fs};

fn main() {
  for argument in env::args().skip(1) {
    if argument == "--schema" {
      let workflow_definition_schema = schema_for!(WorkflowDefinition);
      println!(
        "{}",
        serde_json::to_string_pretty(&workflow_definition_schema).unwrap()
      );
      continue;
    }

    match fs::read_to_string(&argument)
      .map(|content| serde_json::from_str::<WorkflowDefinition>(&content).map_err(|e| e.into()))
      .flatten()
    {
      Ok(workflow_definition) => {
        match &workflow_definition {
          WorkflowDefinition::Version1_8(workflow_definition) => {
            println!("Workflow: {}", workflow_definition.identifier);
            println!(
              "  Version: {}.{}.{}",
              workflow_definition.version_major,
              workflow_definition.version_minor,
              workflow_definition.version_micro
            );
            println!("  Steps:");
            for step in &workflow_definition.steps {
              println!("    - {}", step.label);
            }
          }
        }

        // validate serialisation
        let content = fs::read_to_string(argument).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();

        if serde_json::to_value(&workflow_definition).unwrap() != value {
          println!("Direct parse");
          println!("{:?}", value);
          println!("Generated from Rust model");
          println!("{:?}", serde_json::to_value(&workflow_definition).unwrap());
        }
      }
      Err(error) => {
        println!("{}", error);
      }
    };
  }
}
