use wishlist::infrastructure::{config::Config, logging};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // Initialize the logging system
    logging::setup_logging();

    // Initialize the HTTP server
    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(server_config).await?;
    http_server.run().await?;
}
