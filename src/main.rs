use axum::{Router, middleware, routing::get};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;

use crate::{auth::auth_middleware, status::get_status};

mod status;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/status", get(get_status));

    let port = env::var("PORT").unwrap_or_else(|_| "1345".to_string());
    let port: u16 = match port.parse() {
        Ok(port) => port,
        Err(_) => panic!("PORT is invalid"),
    };

    // TODO check if port is in use
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
