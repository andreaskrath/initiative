use axum::{
    Router,
    body::Body,
    extract::Path,
    http::{HeaderValue, StatusCode, header},
    response::{Html, Response},
    routing::get,
};
use include_dir::{Dir, include_dir};
use tokio::net::TcpListener;

static ASSETS_DIR: Dir<'static> = include_dir!("./view/dist/assets");
static INDEX: Html<&'static str> = Html(include_str!("../../view/dist/index.html"));

fn main() {
    async_main();
}

#[tokio::main]
async fn async_main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/assets/{*path}", get(assets));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to port 8080");
    axum::serve(listener, app)
        .await
        .expect("failed to start server on port 8080");
}

async fn index() -> Html<&'static str> {
    INDEX
}

async fn assets(Path(path): Path<String>) -> Response {
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

fn get_content_type(path: &str) -> &str {
    let mut iter = path.split('.');
    match iter.next_back() {
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        _ => "text/plain",
    }
}
