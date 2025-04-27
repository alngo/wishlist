#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEmail(String);

impl From<&str> for UserEmail {
    fn from(value: &str) -> Self {
        UserEmail(value.to_string())
    }
}
