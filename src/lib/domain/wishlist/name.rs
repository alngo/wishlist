#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WishlistName(String);

impl From<&str> for WishlistName {
    fn from(value: &str) -> Self {
        WishlistName(value.to_string())
    }
}
