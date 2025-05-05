use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn new(password: &str) -> Result<Self, UserPasswordInvalidError> {
        Ok(UserPassword(password.to_string()))
    }
}

impl From<&str> for UserPassword {
    fn from(value: &str) -> Self {
        UserPassword(value.to_string())
    }
}

impl Display for UserPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

#[derive(Clone, Debug, Error)]
#[error("Password is invalid")]
pub struct UserPasswordInvalidError;
