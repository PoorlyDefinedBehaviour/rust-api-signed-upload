use std::collections::HashMap;

/// Contains data that identifies the command, query or request
/// that's being executed.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Context {
  pub data: HashMap<String, String>,
}

impl From<HashMap<String, String>> for Context {
  fn from(input: HashMap<String, String>) -> Self {
    Self { data: input }
  }
}
