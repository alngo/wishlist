/*
Module `create_author` specifies an HTTP handler for creating a new [User], and the
associated data structures.
*/

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::application::UseCases;
use crate::domain::{
    CreateUserError, CreateUserRequest, User, UserEmail, UserEmailInvalidError, UserPassword,
    UserPasswordInvalidError,
};
use crate::interface::http::AppState;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<CreateUserError> for ApiError {
    fn from(e: CreateUserError) -> Self {
        match e {
            CreateUserError::Duplicate { email } => {
                Self::UnprocessableEntity(format!("User with email {} already exists", email))
            }
            CreateUserError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateUserHttpRequestError> for ApiError {
    fn from(e: ParseCreateUserHttpRequestError) -> Self {
        let message = match e {
            ParseCreateUserHttpRequestError::EmailAddress(cause) => {
                format!("email address {} is invalid", cause.invalid_email)
            }
            ParseCreateUserHttpRequestError::Password(_) => {
                format!("password is invalid")
            }
        };

        Self::UnprocessableEntity(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The response body data field for successful [User] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateUserResponseData {
    id: String,
}

impl From<&User> for CreateUserResponseData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
        }
    }
}

/// The body of an [User] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateUserHttpRequestBody {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateUserHttpRequestError {
    #[error(transparent)]
    EmailAddress(#[from] UserEmailInvalidError),
    #[error(transparent)]
    Password(#[from] UserPasswordInvalidError),
}

impl CreateUserHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    fn try_into_domain(self) -> Result<CreateUserRequest, ParseCreateUserHttpRequestError> {
        let email = UserEmail::new(&self.email)?;
        let password = UserPassword::new(&self.password)?;
        Ok(CreateUserRequest::new(email, password))
    }
}

/// Create a new [User].
///
/// # Responses
///
/// - 201 Created: the [User] was successfully created.
/// - 422 Unprocessable entity: An [User] with the same name already exists.
pub async fn create_user<UC: UseCases>(
    State(state): State<AppState<UC>>,
    Json(body): Json<CreateUserHttpRequestBody>,
) -> Result<ApiSuccess<CreateUserResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .services
        .create_user(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref user| ApiSuccess::new(StatusCode::CREATED, user.into()))
}

#[cfg(test)]
mod tests {
    use std::{future, sync::Arc};

    use uuid::Uuid;

    use crate::{
        application::Service,
        domain::{MockUserService, MockWishlistService},
    };

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_user_success() {
        let email = UserEmail::new("a@b.c").unwrap();
        let password = UserPassword::new("password").unwrap();
        let id = Uuid::now_v7();

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_create_user()
            .return_once(move |req| {
                let user = User::new(id, req.email().clone(), req.password().clone());
                Box::pin(future::ready(Ok(user)))
            });
        let mock_wish_service = MockWishlistService::new();
        let service = Service::new(mock_user_service, mock_wish_service);
        let state = axum::extract::State(AppState {
            services: Arc::new(service),
        });
        let body = axum::extract::Json(CreateUserHttpRequestBody {
            email: email.to_string(),
            password: password.to_string(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateUserResponseData { id: id.to_string() },
        );

        let actual = create_user(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_author to succeed, but got {:?}",
            actual
        );

        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        )
    }
}
