use std::future::Future;

#[cfg(test)]
use mockall::automock;
use thiserror::Error;
use uuid::Uuid;

use super::{Item, ItemImageUrl, ItemLinkUrl, ItemPrice, ItemTitle};

/// The [ItemService] trait defines the contract for item-related operations.
#[cfg_attr(test, automock)]
#[allow(refining_impl_trait)]
pub trait ItemService {
    /// Creates a new item with the provided request.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateItemRequest` containing the item's details.
    ///
    /// # Returns
    /// - `Ok(item)` if the item is created successfully.
    /// - `Err(CreateItemError)` if there is an error during item creation.
    ///
    /// # Errors
    /// - [CreateItemError::Duplicate] if an item with the same URL already exists.
    /// - [CreateItemError::Unkown] for any other errors that may occur during item creation.
    fn create_item(
        &self,
        req: &CreateItemRequest,
    ) -> impl Future<Output = Result<Item, CreateItemError>>;
}

/// The [CreateItemRequest] struct represents a request to create a new [Item].
#[derive(Debug, Clone)]
pub struct CreateItemRequest {
    title: ItemTitle,
    link_url: ItemLinkUrl,
    image_url: ItemImageUrl,
    price: ItemPrice,
}

impl CreateItemRequest {
    pub fn new(
        title: ItemTitle,
        link_url: ItemLinkUrl,
        image_url: ItemImageUrl,
        price: ItemPrice,
    ) -> Self {
        Self {
            title,
            link_url,
            image_url,
            price,
        }
    }

    pub fn title(&self) -> &ItemTitle {
        &self.title
    }

    pub fn link_url(&self) -> &ItemLinkUrl {
        &self.link_url
    }

    pub fn image_url(&self) -> &ItemImageUrl {
        &self.image_url
    }

    pub fn price(&self) -> &ItemPrice {
        &self.price
    }
}

#[derive(Debug, Error)]
pub enum CreateItemError {
    #[error("Item with already exist")]
    Duplicate,
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}

/// The [FindItemByIdRequest] struct represents a request to find an item by their ID.
#[derive(Debug, Clone)]
pub struct FindItemByIdRequest {
    id: Uuid,
}

impl FindItemByIdRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Debug, Error)]
pub enum FindItemByIdError {
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}
