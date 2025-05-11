use std::{future::Future, sync::Arc};

use crate::domain::{
    CreateUserError, CreateUserRequest, CreateWishlistError, CreateWishlistRequest, User,
    UserService, Wishlist, WishlistService,
};

pub mod user;
pub mod wishlist;

pub trait UseCases: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, CreateUserError>> + Send;
    fn create_wishlist(
        &self,
        req: &CreateWishlistRequest,
    ) -> impl Future<Output = Result<Wishlist, CreateWishlistError>> + Send;
}

pub struct Service<U, W>
where
    U: UserService,
    W: WishlistService,
{
    user_service: Arc<U>,
    wish_service: Arc<W>,
}

impl<U, W> Service<U, W>
where
    U: UserService,
    W: WishlistService,
{
    pub fn new(user_service: U, wish_service: W) -> Self {
        Self {
            user_service: Arc::new(user_service),
            wish_service: Arc::new(wish_service),
        }
    }
}

impl<U, W> Clone for Service<U, W>
where
    U: UserService,
    W: WishlistService,
{
    fn clone(&self) -> Self {
        Self {
            user_service: self.user_service.clone(),
            wish_service: self.wish_service.clone(),
        }
    }
}

impl<U, W> UseCases for Service<U, W>
where
    U: UserService,
    W: WishlistService,
{
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
        let result = self.user_service.create_user(req).await;
        result
    }

    async fn create_wishlist(
        &self,
        req: &CreateWishlistRequest,
    ) -> Result<Wishlist, CreateWishlistError> {
        let result = self.wish_service.create_wishlist(req).await;
        result
    }
}
