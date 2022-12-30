use std::{
  fmt::{Debug, Display},
  ops::Deref,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
  #[error("the email is invalid: {0:?}")]
  InvalidEmail(String),
}

pub struct Email(String);

impl Email {
  pub fn expose(&self) -> &str {
    &self.0
  }
}

impl Deref for Email {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Debug for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0.find('@') {
      None => write!(f, "INVALID EMAIL: {:}", self.0),
      Some(domain_starts_after) => write!(f, "{}", &self.0[0..domain_starts_after]),
    }
  }
}

impl Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl TryFrom<&str> for Email {
  type Error = EmailError;

  fn try_from(input: &str) -> Result<Self, Self::Error> {
    if validator::validate_email(input) {
      Ok(Email(input.to_owned()))
    } else {
      Err(EmailError::InvalidEmail(input.to_owned()))
    }
  }
}

impl TryFrom<String> for Email {
  type Error = EmailError;

  fn try_from(input: String) -> Result<Self, Self::Error> {
    if validator::validate_email(&input) {
      Ok(Email(input))
    } else {
      Err(EmailError::InvalidEmail(input))
    }
  }
}
