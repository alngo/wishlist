use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use super::{
    Item,
    CreateItemRequest, CreateItemError,
    FindItemByIdRequest, FindItemByIdError,
};

/// The [ItemRepoisitory] trait defines the contract for item-related data operations.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ItemRepository {
    /// Saves a new item to the repository.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateItemRequest` containing the item's details.
    ///
    /// # Returns
    /// - `Ok(item)` if the item is created successfully.
    ///
    /// # Errors
    /// - [CreateItemError::Duplicate] if a item with the same url already exists.
    /// - [CreateItemError::Unkown] for any other errors that may occur during item creation.
    async fn save(&self, req: &CreateItemRequest) -> Result<Item, CreateItemError>;
    /// Finds an item by its ID.
    ///
    /// # Arguments
    /// * `req` - A reference to a `FindItemByIdRequest` containing the item's ID.
    ///
    /// # Returns
    /// - `Ok(Some(item))` if an item with the given ID exists.
    /// - `Ok(None)` if no item with the given ID exists.
    ///
    /// # Errors
    /// - [FindItemByIdError::Unkown] for any other errors that may occur during the search.
    async fn find_item_by_id(&self, req: &FindItemByIdRequest) -> Result<Option<Item>, FindItemByIdError>;
}
