use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use super::{
    CreateUserError, CreateUserRequest, FindUserByEmailError, FindUserByEmailRequest,
    FindUserByIdError, FindUserByIdRequest, User,
};

/// The [UserRepository] trait defines the contract for user-related data operations.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository {
    /// Saves a new user to the repository.
    ///
    /// # Arguments
    /// * `req` - A reference to a `CreateUserRequest` containing the user's email and password.
    ///
    /// # Returns
    /// - `Ok(user)` if the user is created successfully.
    ///
    /// # Errors
    /// - [CreateUserError::Duplicate] if a user with the same email already exists.
    /// - [CreateUserError::Unkown] for any other errors that may occur during user creation.
    async fn save(&self, req: &CreateUserRequest) -> Result<User, CreateUserError>;
    /// Finds a user by their email address.
    ///
    /// # Arguments
    /// * `req` - A reference to a `FindUserByEmailRequest` containing the user's email.
    ///
    /// # Returns
    /// - `Ok(Some(user))` if a user with the given email exists.
    /// - `Ok(None)` if no user with the given email exists.
    ///
    /// # Errors
    /// - [FindUserByEmailError::Unkown] for any other errors that may occur during the search.
    async fn find_user_by_email(
        &self,
        req: &FindUserByEmailRequest,
    ) -> Result<Option<User>, FindUserByEmailError>;
    /// Finds a user by their ID.
    ///
    /// # Arguments
    /// * `id` - A reference to a string containing the user's ID.
    ///
    /// # Returns
    /// - `Ok(Some(user))` if a user with the given ID exists.
    /// - `Ok(None)` if no user with the given ID exists.
    ///
    /// # Errors
    /// - [FindUserByIdError::Unkown] for any other errors that may occur during the search.
    async fn find_user_by_id(
        &self,
        id: &FindUserByIdRequest,
    ) -> Result<Option<User>, FindUserByIdError>;
}
