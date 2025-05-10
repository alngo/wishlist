use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::domain::{
    CreateItemError, CreateItemRequest, FindItemByIdError, FindItemByIdRequest, Item,
    ItemRepository,
};

pub struct InMemoryItemRepository {
    items: Mutex<HashMap<Uuid, Item>>,
}

impl InMemoryItemRepository {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl ItemRepository for InMemoryItemRepository {
    async fn save(&self, _req: &CreateItemRequest) -> Result<Item, CreateItemError> {
        todo!()
    }
    async fn find_item_by_id(
        &self,
        _req: &FindItemByIdRequest,
    ) -> Result<Option<Item>, FindItemByIdError> {
        todo!()
    }
}
