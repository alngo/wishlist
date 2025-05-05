use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use super::{Wishlist, WishlistName};

/// The [WishlistService] trait defines the contract for wishlist-related operations.
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WishlistService: Send + Sync + 'static {
    /// Creates a new wishlist with the provided request.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateWishlistRequest` containing the wishlist's details.
    ///
    /// # Returns
    /// - `Ok(wishlist)` if the wishlist is created successfully.
    ///
    /// # Errors
    /// - [CreateWishlistError::OwnerIdDoesNotExist] if the owner ID does not exist.
    /// - [CreateWishlistError::Unkown] for any other errors that may occur during wishlist
    /// creation.
    async fn create_wishlist(
        &self,
        req: &CreateWishlistRequest,
    ) -> Result<Wishlist, CreateWishlistError>;
}

pub struct CreateWishlistRequest {
    owner_id: Uuid,
    name: WishlistName,
    private: bool,
}

impl CreateWishlistRequest {
    pub fn new(owner_id: Uuid, name: WishlistName, private: bool) -> Self {
        Self {
            owner_id,
            name,
            private,
        }
    }

    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    pub fn name(&self) -> &WishlistName {
        &self.name
    }

    pub fn private(&self) -> bool {
        self.private
    }
}

#[derive(Debug, Error)]
pub enum CreateWishlistError {
    #[error("Owner with id {id} does not exist")]
    OwnerIdDoesNotExist { id: Uuid },
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}
