use axum::{routing::post, Router};
use create_user::create_user;

use crate::application::UseCases;

use super::AppState;

pub mod create_user;

pub fn api_routes<UC: UseCases>() -> Router<AppState<UC>> {
    Router::new().route("/authors", post(create_user::<UC>))
}
