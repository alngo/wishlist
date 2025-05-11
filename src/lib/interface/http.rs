mod handlers;
mod responses;

use crate::application::UseCases;
use anyhow::Context;
use axum::{
    handler::Handler,
    routing::{get, post},
};
use handlers::{api_routes, create_user::create_user};
use std::sync::Arc;
use tokio::net;

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
struct AppState<UC: UseCases> {
    services: Arc<UC>,
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
            .layer(trace_layer)
            .route("/health_check", get(|| async { "OK" }))
            .nest("/api", api_routes())
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
        application::Service,
        domain::{MockUserService, MockWishlistService},
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
        let user_service = MockUserService::new();
        let wish_service = MockWishlistService::new();
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
