use axum::{
    body::Body,
    extract::Path,
    http::{HeaderValue, Request, StatusCode, header},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use include_dir::{Dir, include_dir};

pub mod spells;

static ASSETS_DIR: Dir<'static> = include_dir!("./view/dist/assets");
static INDEX: Html<&'static str> = Html(include_str!("../../view/dist/index.html"));

pub async fn index() -> Html<&'static str> {
    INDEX
}

pub async fn assets(Path(path): Path<String>) -> Response {
    let path = path.trim_start_matches('/');
    let content_type = get_content_type(path);

    let content_type_value = match HeaderValue::try_from(content_type) {
        Ok(v) => v,
        Err(err) => todo!(),
    };

    let (status_code, body) = match ASSETS_DIR.get_file(path) {
        Some(file) => (StatusCode::OK, Body::from(file.contents())),
        None => (StatusCode::NOT_FOUND, Body::empty()),
    };

    Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, content_type_value)
        .body(body)
        .expect("failed to build response")
}

/// Redirects to index when an invalid method is used.
pub async fn method_not_allowed_handler(request: Request<Body>, next: Next) -> Response {
    let response = next.run(request).await;

    if response.status() == StatusCode::METHOD_NOT_ALLOWED {
        // Call your fallback index handler directly
        return index().await.into_response();
    }

    response
}

fn get_content_type(path: &str) -> &str {
    let mut iter = path.split('.');
    match iter.next_back() {
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        _ => "text/plain",
    }
}
