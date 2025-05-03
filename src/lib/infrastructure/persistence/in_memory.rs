mod user;

mod wishlist {
    use std::{collections::HashMap, sync::Mutex};

    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::domain::{Wishlist, WishlistRepository};

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

    #[async_trait]
    impl WishlistRepository for InMemoryWishlistRepository {}
}
