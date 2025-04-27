#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDescription(String);

impl From<&str> for ItemDescription {
    fn from(value: &str) -> Self {
        ItemDescription(value.to_string())
    }
}
