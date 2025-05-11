use std::future::Future;

use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use super::{User, UserEmail, UserPassword};

/// The [UserService] trait defines the contract for user-related operations.
#[cfg_attr(test, automock)]
#[allow(refining_impl_trait)]
pub trait UserService: Send + Sync + 'static {
    /// Creates a new user with the provided request.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateUserRequest` containing the user's email and password.
    /// # Returns
    /// - `Ok(user)` if the user is created successfully.
    /// - `Err(CreateUserError)` if there is an error during user creation.
    ///
    /// # Errors
    /// - [CreateUserError::Duplicate] if a user with the same email already exists.
    /// - [CreateUserError::Unknown] for any other errors that may occur during user creation.
    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, CreateUserError>> + Send;
}

/// The [CreateUserRequest] struct represents a request to create a new [User].
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
    Unknown(#[from] anyhow::Error),
}

/// The [FindUserByEmailRequest] struct represents a request to find a user by their email address.
#[derive(Debug, Clone)]
pub struct FindUserByEmailRequest {
    email: UserEmail,
}

impl FindUserByEmailRequest {
    pub fn new(email: UserEmail) -> Self {
        Self { email }
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }
}

#[derive(Debug, Error)]
pub enum FindUserByEmailError {
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}

/// The [FindUserByIdRequest] struct represents a request to find a user by their ID.
#[derive(Debug, Clone)]
pub struct FindUserByIdRequest {
    id: Uuid,
}

impl FindUserByIdRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Debug, Error)]
pub enum FindUserByIdError {
    #[error(transparent)]
    Unkown(#[from] anyhow::Error),
}
