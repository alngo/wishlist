use async_trait::async_trait;

use crate::domain::{CreateUserError, CreateUserRequest, User, UserRepository, UserService};

pub struct Service<'u, U>
where
    U: UserRepository,
{
    user_repository: &'u U,
}

impl<'u, U> Service<'u, U>
where
    U: UserRepository,
{
    pub fn new(user_repository: &'u U) -> Self {
        Self { user_repository }
    }
}

#[async_trait(?Send)]
impl<'u, U> UserService for Service<'u, U>
where
    U: UserRepository,
{
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
        let result = self.user_repository.save(req).await;
        result
    }
}
