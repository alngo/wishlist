use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{CreateWishlistError, CreateWishlistRequest, Wishlist, WishlistRepository};

pub struct InMemoryWishlistRepository {
    wishlists: Mutex<HashMap<Uuid, Wishlist>>,
}

impl InMemoryWishlistRepository {
    pub fn new() -> Self {
        Self {
            wishlists: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait(?Send)]
impl WishlistRepository for InMemoryWishlistRepository {
    async fn save(&self, _req: &CreateWishlistRequest) -> Result<Wishlist, CreateWishlistError> {
        todo!()
    }
}
