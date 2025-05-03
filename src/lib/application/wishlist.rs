use async_trait::async_trait;

use crate::domain::{
    CreateWishlistError, CreateWishlistRequest, FindUserByIdRequest, UserRepository, Wishlist,
    WishlistRepository,
};

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

#[async_trait(?Send)]
impl<'u, 'w, U, W> crate::domain::WishlistService for Service<'u, 'w, U, W>
where
    U: UserRepository,
    W: WishlistRepository,
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
                return Err(CreateWishlistError::Unkown(err.into()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::domain::{MockUserRepository, MockWishlistRepository, User, WishlistService};

    #[tokio::test]
    async fn test_create_wishlist() {
        let id = Uuid::now_v7();
        let req = CreateWishlistRequest::new(id, "".into(), true);
        let mut user_mock_repo = MockUserRepository::new();
        user_mock_repo
            .expect_find_user_by_id()
            .returning(move |_| Ok(Some(User::new(id, "".into(), "".into()))));
        let mut wish_mock_repo = MockWishlistRepository::new();
        wish_mock_repo
            .expect_save()
            .returning(move |_| Ok(Wishlist::new(id, id, "".into(), "".into(), true)));

        let wish_service = Service::new(&user_mock_repo, &wish_mock_repo);

        let result = wish_service.create_wishlist(&req).await;
        assert!(result.is_ok());
    }
}
