use std::future::Future;

#[cfg(test)]
use mockall::automock;

use super::{CreateItemError, CreateItemRequest, FindItemByIdError, FindItemByIdRequest, Item};

/// The [ItemRepoisitory] trait defines the contract for item-related data operations.
#[cfg_attr(test, automock)]
#[allow(refining_impl_trait)]
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
    fn save(
        &self,
        req: &CreateItemRequest,
    ) -> impl Future<Output = Result<Item, CreateItemError>> + Send;
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
    fn find_item_by_id(
        &self,
        req: &FindItemByIdRequest,
    ) -> impl Future<Output = Result<Option<Item>, FindItemByIdError>> + Send;
}
