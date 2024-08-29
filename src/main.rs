mod util;
mod directory_interceptor;

use axum::routing::get_service;
use axum::Router;
use serde::Serialize;
use std::future::Future;
use tower::{Layer, Service};
use tower_http::services::ServeDir;

use crate::directory_interceptor::DirectoryInterceptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let static_files = get_service(ServeDir::new("."))
        .handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        })
        .layer(DirectoryInterceptor);

    let app = Router::new().nest_service("/", static_files);

    let addr = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
