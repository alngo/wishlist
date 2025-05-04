use wishlist::{infrastructure::{config::Config, logging}, interface::http::{HttpServer, HttpServerConfig}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Initialize the logging system
    logging::setup_logging();

    let config = Config::load()?;
    tracing::info!("Loaded configuration");

    // Initialize the HTTP server
    let server_config = HttpServerConfig {
        port: config.server.port,
        host: config.server.host,
    };
    let http_server = HttpServer::new(server_config).await?;
    http_server.run().await
}
