use std::net::{SocketAddr, TcpListener};

use wishlist::interface::http::{HttpServer, HttpServerConfig};

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let local_addr = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port")
        .local_addr()
        .unwrap();
    let address = SocketAddr::new(local_addr.ip(), local_addr.port());

    let server_config = HttpServerConfig {
        port: local_addr.port(),
        host: local_addr.ip().to_string(),
    };

    let http_server = HttpServer::new(server_config)
        .await
        .expect("Failed to create HttpServer");

    tokio::spawn(http_server.run());

    TestApp {
        address: format!("http://{}", address),
    }
}

#[tokio::test]
async fn test_health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}
