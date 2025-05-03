use std::{collections::HashMap, sync::Mutex};

use crate::domain::{CreateUserError, CreateUserRequest, FindUserByEmailError, FindUserByEmailRequest, FindUserByIdError, FindUserByIdRequest, User, UserRepository};
use async_trait::async_trait;
use uuid::Uuid;

/// The [InMemoryUserRepository] struct is an in-memory implementation of the [UserRepository]
/// trait.
/// It uses a `Mutex` to provide thread-safe access to the underlying data structure.
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
        let mut users = self.users.lock().unwrap();
        if users.values().any(|user| user.email() == req.email()) {
            return Err(CreateUserError::Duplicate {
                email: req.email().clone(),
            });
        }
        let id = Uuid::now_v7();
        let user = User::new(id, req.email().clone(), req.password().clone());
        users.insert(id, user.clone());
        Ok(user)
    }

    async fn find_user_by_email(
        &self,
        req: &FindUserByEmailRequest,
    ) -> Result<Option<User>, FindUserByEmailError> {
        let users = self.users.lock().unwrap();
        let user = users.values().find(|user| user.email() == req.email());
        Ok(user.cloned())
    }

    async fn find_user_by_id(
        &self,
        id: &FindUserByIdRequest,
    ) -> Result<Option<User>, FindUserByIdError> {
        let users = self.users.lock().unwrap();
        let user = users.get(&id.id());
        Ok(user.cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let req = CreateUserRequest::new("".into(), "".into());
        let repository = InMemoryUserRepository::new();

        let result = repository.save(&req).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email(), req.email());
        assert_eq!(user.password(), req.password());
    }

    #[tokio::test]
    async fn test_create_duplicate_user() {
        let req = CreateUserRequest::new("".into(), "".into());
        let repository = InMemoryUserRepository::new();

        let _ = repository.save(&req).await;
        let result = repository.save(&req).await;

        assert!(result.is_err());
        if let Err(CreateUserError::Duplicate { email }) = result {
            assert_eq!(email, req.email().clone());
        } else {
            panic!("Expected CreateUserError::Duplicate");
        }
    }

    #[tokio::test]
    async fn test_find_user_by_email() {
        let req = CreateUserRequest::new("a@b.c".into(), "".into());
        let repository = InMemoryUserRepository::new();

        let user = repository.save(&req).await.unwrap();
        let find_req = FindUserByEmailRequest::new(user.email().clone());

        let result = repository.find_user_by_email(&find_req).await;
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert_eq!(found_user.as_ref().unwrap().email(), user.email());

        let find_req = FindUserByEmailRequest::new("notfound@b.c".into());
        let result = repository.find_user_by_email(&find_req).await;
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_none());
    }

    #[tokio::test]
    async fn test_find_user_by_id() {
        let req = CreateUserRequest::new("a@b.c".into(), "".into());
        let repository = InMemoryUserRepository::new();

        let user = repository.save(&req).await.unwrap();
        let find_req = FindUserByIdRequest::new(user.id().clone());

        let result = repository.find_user_by_id(&find_req).await;
        assert!(result.is_ok());

        let found_user = result.unwrap();
        assert_eq!(found_user.as_ref().unwrap().id(), user.id());

        let find_req = FindUserByIdRequest::new(Uuid::new_v4());
        let result = repository.find_user_by_id(&find_req).await;
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_none());
    }
}
