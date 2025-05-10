use std::sync::Arc;

use crate::domain::{CreateUserError, CreateUserRequest, User, UserRepository, UserService};

pub struct Service<U>
where
    U: UserRepository,
{
    user_repository: Arc<U>,
}

impl<U> Clone for Service<U>
where
    U: UserRepository,
{
    fn clone(&self) -> Self {
        Self {
            user_repository: self.user_repository.clone(),
        }
    }
}

impl<U> Service<U>
where
    U: UserRepository,
{
    pub fn new(user_repository: Arc<U>) -> Self {
        Self { user_repository }
    }
}

impl<U> UserService for Service<U>
where
    U: UserRepository + Send + Sync + 'static,
{
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
        if req.email().to_string().is_empty() {
            return self
                .user_repository
                .save(&CreateUserRequest::new("".into(), "".into()))
                .await;
        }
        self.user_repository.save(req).await
    }
}

#[cfg(test)]
mod tests {
    use std::future;

    use uuid::Uuid;

    use super::*;
    use crate::domain::MockUserRepository;

    #[tokio::test]
    async fn test_create_user() {
        let id = Uuid::now_v7();
        let req = CreateUserRequest::new("".into(), "".into());
        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_save()
            .returning(move |_| Box::pin(future::ready(Ok(User::new(id, "".into(), "".into())))));
        let user_service = Service::new(Arc::new(mock_repo));

        let result = user_service.create_user(&req).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.id(), &id);
        assert_eq!(user.email(), req.email());
        assert_eq!(user.password(), req.password());
    }
}
