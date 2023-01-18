use super::value_objects::{password::PasswordError, email::EmailError};

#[derive(Debug, PartialEq, Eq)]
pub struct ValidationError {
  pub name: String,
  pub message: String
}

impl From<EmailError> for ValidationError {
    fn from(input: EmailError) -> Self {
        match input {
            EmailError::InvalidEmail(_) => Self {
                name: "email".to_owned(),
                message: input.to_string()
            }
        }
    }
}

impl From<PasswordError> for ValidationError {
    fn from(input: PasswordError) -> Self {
        match input {
            PasswordError::InvalidPassword => Self {
                name: "password".to_owned(),
                message: "invalid password".to_owned()
            }
        }
    }
}
