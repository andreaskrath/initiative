mod handlers;
mod repositories;
mod types;

use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use handlers::spells;

fn main() {
    async_main();
}

#[tokio::main]
async fn async_main() {
    let db_connection_string = env::var("DB_CONNECTION_STRING")
        .expect("DB_CONNECTION_STRING environment variable is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_string)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/assets/{*path}", get(handlers::assets))
        .route("/api/spells/create", post(spells::create))
        .fallback(handlers::index)
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:5173")
        .await
        .expect("failed to bind to port 8080");
    axum::serve(listener, app)
        .await
        .expect("failed to start server on port 5173");
}
