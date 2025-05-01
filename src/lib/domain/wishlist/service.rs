use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use super::{Wishlist, WishlistName};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WishlistService {
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

#[derive(Debug, Error)]
pub enum CreateWishlistError {
    #[error("Owner with id {id} does not exist")]
    OwnerIdDoesNotExist { id: Uuid },
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}
