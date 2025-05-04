use anyhow::Context;
use tokio::net;

use axum::routing::get;

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

impl HttpServer {
    pub async fn new(config: HttpServerConfig) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let router = axum::Router::new()
            .route("/health_check", get(|| async { "OK" }))
            // .nest("/api", api_routes())
            .layer(trace_layer);

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

// fn api_routes<BS: BlogService>() -> Router<AppState<BS>> {
//     Router::new().route("/authors", post(create_author::<BS>))
// }
