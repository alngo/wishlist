use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WishlistName(String);

impl From<&str> for WishlistName {
    fn from(value: &str) -> Self {
        WishlistName(value.to_string())
    }
}

impl Display for WishlistName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
