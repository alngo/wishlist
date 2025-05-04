use std::sync::Arc;

use wishlist::{
    application::{user, wishlist as wish},
    infrastructure::{
        config::Config,
        logging,
        persistence::in_memory::{
            user::InMemoryUserRepository, wishlist::InMemoryWishlistRepository,
        },
    },
    interface::http::{HttpServer, HttpServerConfig},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Initialize the logging system
    logging::setup_logging();

    let config = Config::load()?;
    tracing::info!("Loaded configuration");

    let user_repo = Arc::new(InMemoryUserRepository::new());
    let user_service = user::Service::new(user_repo.clone());

    let wish_repo = Arc::new(InMemoryWishlistRepository::new());
    let wish_service = wish::Service::new(user_repo.clone(), wish_repo.clone());

    // Initialize the HTTP server
    let server_config = HttpServerConfig {
        port: config.server.port,
        host: config.server.host,
    };
    let http_server = HttpServer::new(user_service, wish_service, server_config).await?;
    http_server.run().await
}
