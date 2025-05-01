use crate::domain::{UserRepository, WishlistRepository};

pub struct Service<'u, 'w, U, W>
where
    U: UserRepository,
    W: WishlistRepository,
{
    user_repository: &'u U,
    wish_repository: &'w W,
}

impl<'u, 'w, U, W> Service<'u, 'w, U, W>
where
    U: UserRepository,
    W: WishlistRepository,
{
    pub fn new(user_repository: &'u U, wish_repository: &'w W) -> Self {
        Self {
            user_repository,
            wish_repository,
        }
    }
}
