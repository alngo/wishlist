use crate::domain::User;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use thiserror::Error;

use super::{UserEmail, UserPassword};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UserService {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError>;
}

#[derive(Debug, Clone)]
pub struct CreateUserRequest {
    email: UserEmail,
    password: UserPassword,
}

impl CreateUserRequest {
    pub fn new(email: UserEmail, password: UserPassword) -> Self {
        Self { email, password }
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }
}

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("User with email {email} already exist")]
    Duplicate { email: UserEmail },
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}
