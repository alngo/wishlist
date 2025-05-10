use std::future::Future;

use crate::domain::wishlist::{CreateWishlistError, CreateWishlistRequest, Wishlist};

#[cfg(test)]
use mockall::automock;

/// The [WishlistRepository] trait defines the contract for wishlist-related data operations.
#[cfg_attr(test, automock)]
#[allow(refining_impl_trait)]
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
    fn save(
        &self,
        req: &CreateWishlistRequest,
    ) -> impl Future<Output = Result<Wishlist, CreateWishlistError>>;
}
