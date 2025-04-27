#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WishlistSlug(String);

impl From<&str> for WishlistSlug {
    fn from(value: &str) -> Self {
        WishlistSlug(value.to_string())
    }
}
