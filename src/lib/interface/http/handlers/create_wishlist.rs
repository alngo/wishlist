/*
Module `create_wishlist` specifies an HTTP handler for creating a new [Wishlist], and the
associated data structures.
*/

use axum::http::StatusCode;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    CreateWishlistError, CreateWishlistRequest, OwnerIdInvalidError, WishlistName,
    WishlistNameInvalidError,
};
use crate::{application::UseCases, domain::Wishlist, interface::http::AppState};

use super::{ApiError, ApiSuccess};

impl From<CreateWishlistError> for ApiError {
    fn from(e: CreateWishlistError) -> Self {
        match e {
            CreateWishlistError::OwnerIdDoesNotExist { id } => {
                Self::UnprocessableEntity(format!("Owner ID {} does not exist", id))
            }
            CreateWishlistError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateWishlistHttpRequestError> for ApiError {
    fn from(e: ParseCreateWishlistHttpRequestError) -> Self {
        let message = match e {
            ParseCreateWishlistHttpRequestError::InvalidOwnerId(cause) => {
                format!("owner id {} is invalid", cause.invalid_owner_id)
            }
            ParseCreateWishlistHttpRequestError::InvalidName(_) => {
                format!("name is invalid")
            }
        };

        Self::UnprocessableEntity(message)
    }
}

/// The response body data field for successful [Wishlist] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateWishlistResponseData {
    pub id: String,
}

impl From<&Wishlist> for CreateWishlistResponseData {
    fn from(wishlist: &Wishlist) -> Self {
        Self {
            id: wishlist.id().to_string(),
        }
    }
}

/// The body of an [Wishlist] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateWishlistHttpRequestBody {
    pub name: String,
    pub owner_id: String,
    pub private: bool,
}

#[derive(Debug, Clone, Error)]
pub enum ParseCreateWishlistHttpRequestError {
    #[error(transparent)]
    InvalidOwnerId(#[from] OwnerIdInvalidError),
    #[error(transparent)]
    InvalidName(#[from] WishlistNameInvalidError),
}

impl CreateWishlistHttpRequestBody {
    /// Converts the HTTP request body into a domain [CreateWishlistRequest].
    pub fn try_into_domain(
        self,
    ) -> Result<CreateWishlistRequest, ParseCreateWishlistHttpRequestError> {
        let owner_id = Uuid::parse_str(&self.owner_id).map_err(|_| OwnerIdInvalidError {
            invalid_owner_id: Uuid::parse_str(&self.owner_id).unwrap_or_default(),
        })?;
        let name = WishlistName::new(&self.name)?;
        let private = self.private;
        Ok(CreateWishlistRequest::new(owner_id, name, private))
    }
}

/// Create a new [Wishlist].
///
/// # Response
///
/// - 201 Created: the [Wishlist] was successfully created.
/// - 422 Unprocessable entity: An [Wishlist] with the same name already exists.
pub async fn create_wishlist<UC: UseCases>(
    State(state): State<AppState<UC>>,
    Json(body): Json<CreateWishlistHttpRequestBody>,
) -> Result<ApiSuccess<CreateWishlistResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .services
        .create_wishlist(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref wish: Wishlist| ApiSuccess::new(StatusCode::CREATED, wish.into()))
}

#[cfg(test)]
mod tests {
    use std::{future, sync::Arc};

    use crate::{
        application::Service,
        domain::{MockUserService, MockWishlistService, WishlistSlug},
    };

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_wishlist_success() {
        let id = Uuid::now_v7();
        let name = WishlistName::from("Test wishlist");
        let private = true;

        let mut mock_wish_service = MockWishlistService::new();
        mock_wish_service
            .expect_create_wishlist()
            .return_once(move |req| {
                let wishlist = Wishlist::new(
                    id,
                    req.owner_id(),
                    req.name().clone(),
                    WishlistSlug::from("Test Wishlist"),
                    req.private(),
                );
                Box::pin(future::ready(Ok(wishlist)))
            });

        let mock_user_service = MockUserService::new();
        let service = Service::new(mock_user_service, mock_wish_service);
        let state = axum::extract::State(AppState {
            services: Arc::new(service),
        });
        let body = axum::extract::Json(CreateWishlistHttpRequestBody {
            name: name.to_string(),
            owner_id: id.to_string(),
            private,
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateWishlistResponseData { id: id.to_string() },
        );
        let actual = create_wishlist(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_wishlist to succeed, but got {:?}",
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
