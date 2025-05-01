use std::fmt::{Display, Formatter};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WishlistSlug(String);

impl From<&str> for WishlistSlug {
    fn from(value: &str) -> Self {
        let base = value.to_lowercase().replace(' ', "-");
        let suffix: String = Uuid::new_v4().to_string()[..8].to_string();
        WishlistSlug(format!("{}-{}", base, suffix))
    }
}

impl Display for WishlistSlug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::WishlistSlug;

    #[test]
    fn slug_from_str() {
        let slug = WishlistSlug::from("TeSt");
        assert!(slug.0.contains("test-"));
    }
}
