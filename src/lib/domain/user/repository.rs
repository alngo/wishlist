use crate::domain::{CreateUserError, CreateUserRequest, User};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository {
    async fn save(&self, req: &CreateUserRequest) -> Result<User, CreateUserError>;
}
