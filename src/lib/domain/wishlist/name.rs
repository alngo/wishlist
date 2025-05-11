use std::fmt::{Display, Formatter};

use thiserror::Error;

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

impl WishlistName {
    pub fn new(name: &str) -> Result<Self, WishlistNameInvalidError> {
        if name.is_empty() {
            return Err(WishlistNameInvalidError {
                invalid_name: WishlistName(name.to_string()),
            });
        }
        Ok(WishlistName(name.to_string()))
    }
}

#[derive(Clone, Debug, Error)]
#[error("Name is invalid")]
pub struct WishlistNameInvalidError {
    pub invalid_name: WishlistName,
}
