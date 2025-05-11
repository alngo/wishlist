use std::sync::Arc;

use crate::domain::{
    CreateWishlistError, CreateWishlistRequest, FindUserByIdRequest, UserRepository, Wishlist,
    WishlistRepository, WishlistService,
};

pub struct Service<U, W>
where
    U: UserRepository,
    W: WishlistRepository,
{
    user_repository: Arc<U>,
    wish_repository: Arc<W>,
}

impl<U, W> Clone for Service<U, W>
where
    U: UserRepository,
    W: WishlistRepository,
{
    fn clone(&self) -> Self {
        Self {
            user_repository: self.user_repository.clone(),
            wish_repository: self.wish_repository.clone(),
        }
    }
}

impl<U, W> Service<U, W>
where
    U: UserRepository,
    W: WishlistRepository,
{
    pub fn new(user_repository: Arc<U>, wish_repository: Arc<W>) -> Self {
        Self {
            user_repository,
            wish_repository,
        }
    }
}

impl<U, W> WishlistService for Service<U, W>
where
    U: UserRepository + Send + Sync + 'static,
    W: WishlistRepository + Send + Sync + 'static,
{
    async fn create_wishlist(
        &self,
        req: &CreateWishlistRequest,
    ) -> Result<Wishlist, CreateWishlistError> {
        match self
            .user_repository
            .find_user_by_id(&FindUserByIdRequest::new(req.owner_id()))
            .await
        {
            Ok(Some(_)) => self.wish_repository.save(req).await,
            Ok(None) => {
                return Err(CreateWishlistError::OwnerIdDoesNotExist { id: req.owner_id() });
            }
            Err(err) => {
                return Err(CreateWishlistError::Unknown(err.into()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future;

    use uuid::Uuid;

    use super::*;
    use crate::domain::{MockUserRepository, MockWishlistRepository, User, WishlistService};

    #[tokio::test]
    async fn test_create_wishlist() {
        let id = Uuid::now_v7();
        let req = CreateWishlistRequest::new(id, "".into(), true);
        let mut user_mock_repo = MockUserRepository::new();
        user_mock_repo.expect_find_user_by_id().returning(move |_| {
            Box::pin(future::ready(Ok(Some(User::new(id, "".into(), "".into())))))
        });
        let mut wish_mock_repo = MockWishlistRepository::new();
        wish_mock_repo.expect_save().returning(move |_| {
            Box::pin(future::ready(Ok(Wishlist::new(
                id,
                id,
                "".into(),
                "".into(),
                true,
            ))))
        });
        let wish_service = Service::new(Arc::new(user_mock_repo), Arc::new(wish_mock_repo));
        let result = wish_service.create_wishlist(&req).await;
        assert!(result.is_ok());
    }
}
