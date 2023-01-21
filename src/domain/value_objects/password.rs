use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("the password is invalid")]
    InvalidPassword
}

pub struct Password(String);

impl Password {
    pub fn expose(&self) -> String {
        self.0.to_owned()
    }
}

impl<'a> TryFrom<&'a str> for Password {
    type Error = PasswordError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let hash = match bcrypt::hash(value, bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(PasswordError::InvalidPassword)
        };

        Ok(Self(hash))
    }
}
