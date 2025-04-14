use axum::{Router, response::Html, routing::get};
use tokio::net::TcpListener;

fn main() {
    async_main();
}

#[tokio::main]
async fn async_main() {
    let app = Router::new().route("/", get(index));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to port 8080");
    axum::serve(listener, app)
        .await
        .expect("failed to start server on port 8080");
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
