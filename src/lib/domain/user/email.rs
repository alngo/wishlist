use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(email: &str) -> Result<Self, UserEmailInvalidError> {
        Ok(UserEmail(email.to_string()))
    }
}

impl From<&str> for UserEmail {
    fn from(value: &str) -> Self {
        UserEmail(value.to_string())
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]
#[error("Email is invalid")]
pub struct UserEmailInvalidError {
    pub invalid_email: UserEmail,
}
