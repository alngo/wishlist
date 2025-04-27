#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemTitle(String);

impl From<&str> for ItemTitle {
    fn from(value: &str) -> Self {
        ItemTitle(value.to_string())
    }
}
