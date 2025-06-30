use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_secret = env::var("AUTH_SECRET");
    let auth_secret = match auth_secret {
        Ok(secret) => secret,
        Err(_) => panic!("AUTH_SECRET is required"),
    };

    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok());
    match auth_header {
        Some(token) if token == auth_secret => Ok(next.run(request).await),
        Some(_) => Err(StatusCode::UNAUTHORIZED),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
