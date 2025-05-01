use crate::domain::Wishlist;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WishlistRepository {
    async fn save(&self, wishlist: &Wishlist) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Wishlist>, String>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Wishlist>, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
