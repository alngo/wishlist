mod user {
    use std::{collections::HashMap, sync::Mutex};

    use crate::domain::{CreateUserError, CreateUserRequest, User, UserRepository};
    use async_trait::async_trait;
    use uuid::Uuid;

    pub struct InMemoryUserRepository {
        users: Mutex<HashMap<Uuid, User>>,
    }

    impl InMemoryUserRepository {
        pub fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn save(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
            Ok(User::new(
                Uuid::now_v7(),
                req.email().clone(),
                req.password().clone(),
            ))
        }
    }
}

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
