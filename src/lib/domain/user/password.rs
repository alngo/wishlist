use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPassword(String);

impl Display for UserPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

impl From<&str> for UserPassword {
    fn from(value: &str) -> Self {
        UserPassword(value.to_string())
    }
}
