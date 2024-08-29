use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use tera::{Context, Tera};
use tower::{Layer, Service};
use crate::fs_server::AppState;
use crate::util::{is_directory, list_files_with_type};

#[derive(Clone)]
pub struct DirectoryInterceptor {
    pub(crate) app_state: Arc<AppState>,
}

impl<S> Layer<S> for DirectoryInterceptor {
    type Service = DirectoryInterceptorService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DirectoryInterceptorService {
            inner,
            app_state: self.app_state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DirectoryInterceptorService<S> {
    inner: S,
    app_state: Arc<AppState>,
}

impl<S> Service<Request<Body>> for DirectoryInterceptorService<S>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let app_state = self.app_state.clone();
        let uri_path = req.uri().path();

        let server_path = &app_state.path;

        let fs_path = server_path.canonicalize().expect("Failed to canonicalize path");
        let relative_path = server_path.join(uri_path.trim_start_matches('/'));

        if is_directory(relative_path.clone()) {
            let mut tera = Tera::default();
            tera.add_template_file("templates/directory.html", Some("directory"))
                .unwrap();

            let file_items = list_files_with_type(relative_path, server_path).unwrap();

            let mut context = Context::new();
            context.insert("directory", fs_path.to_str().unwrap());
            context.insert("directory_url", "");
            context.insert("file_items", &file_items);

            let rendered = tera.render("directory", &context).unwrap();

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from(rendered))
                .unwrap();

            Box::pin(async move { Ok(response) })
        } else {
            let mut inner = self.inner.clone();
            let fut = inner.call(req);

            Box::pin(async move {
                let response = fut.await?;
                Ok(response)
            })
        }
    }
}
