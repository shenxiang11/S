use std::path::PathBuf;
use std::sync::Arc;
use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;
use crate::directory_interceptor::DirectoryInterceptor;

#[derive(Clone, Debug)]
pub struct AppState {
    pub(crate) path: PathBuf,
}

pub async fn start_file_server(path: PathBuf, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let shared_state = AppState {
        path: path.clone(),
    };

    let shared_state = Arc::new(shared_state);

    let static_files = get_service(ServeDir::new(path))
        .with_state(shared_state.clone())
        .handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        })
        .layer(DirectoryInterceptor {
            app_state: shared_state.clone(),
        });

    let app = Router::new().nest_service("/", static_files);

    let addr = &format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
