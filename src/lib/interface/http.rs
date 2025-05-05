use std::sync::Arc;

use anyhow::Context;
use tokio::net;

use axum::routing::get;

use crate::application::UseCases;

pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

impl HttpServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

#[derive(Debug, Clone)]
struct AppState<S: UseCases> {
    services: Arc<S>,
}

impl HttpServer {
    pub async fn new(services: impl UseCases, config: HttpServerConfig) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let app_state = AppState {
            services: Arc::new(services),
        };

        let router = axum::Router::new()
            .route("/health_check", get(|| async { "OK" }))
            // .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(app_state);

        let listener = net::TcpListener::bind((config.host.as_str(), config.port))
            .await
            .with_context(|| format!("Failed to bind to {}:{}", config.host, config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{user, wishlist as wish, Service},
        domain::{MockUserRepository, MockWishlistRepository},
    };

    use super::*;
    use std::net::{SocketAddr, TcpListener};

    async fn spawn_app() -> String {
        let local_addr = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind a random port")
            .local_addr()
            .unwrap();
        let address = SocketAddr::new(local_addr.ip(), local_addr.port());
        let server_config = HttpServerConfig {
            port: local_addr.port(),
            host: local_addr.ip().to_string(),
        };
        let user_repo = Arc::new(MockUserRepository::new());
        let wish_repo = Arc::new(MockWishlistRepository::new());
        let user_service = user::Service::new(user_repo.clone());
        let wish_service = wish::Service::new(user_repo.clone(), wish_repo.clone());
        let services = Service::new(user_service, wish_service);
        let http_server = HttpServer::new(services, server_config)
            .await
            .expect("Failed to create HttpServer");
        tokio::spawn(http_server.run());
        format!("http://{}", address)
    }

    #[tokio::test]
    async fn test_health_check() {
        let address = spawn_app().await;
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(2), response.content_length());
    }
}
