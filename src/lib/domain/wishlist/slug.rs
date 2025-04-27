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
