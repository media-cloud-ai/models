#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod workflow_definition;

pub(crate) fn is_false(v: &bool) -> bool {
  !*v
}
