use crate::domain::wishlist::{CreateWishlistError, CreateWishlistRequest, Wishlist};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

/// The [WishlistRepository] trait defines the contract for wishlist-related data operations.
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WishlistRepository {
    /// Saves a new wishlist to the repository.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateWishlistRequest` containing the wishlist's details.
    ///
    /// # Returns
    /// - `Ok(wishlist)` if the wishlist is created successfully.
    ///
    /// # Errors
    /// - [CreateWishlistError::OwnerIdDoesNotExist] if the owner ID does not exist.
    /// - [CreateWishlistError::Unkown] for any other errors that may occur during wishlist creation.
    async fn save(&self, req: &CreateWishlistRequest) -> Result<Wishlist, CreateWishlistError>;
}
