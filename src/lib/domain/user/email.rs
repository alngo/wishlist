use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct UserEmail(String);

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
