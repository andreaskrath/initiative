use axum::{
    Router,
    routing::{get, post, put},
};
use initiative::{encounter, monster, spell};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

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
        .route("/assets/{*path}", get(initiative::assets))
        .route("/images/{*path}", get(initiative::images))
        .route("/api/spells/create", post(spell::handler::create))
        .route("/api/spells", get(spell::handler::get))
        .route("/api/monsters/create", post(monster::handler::create))
        .route("/api/monsters", get(monster::handler::get))
        .route("/api/encounters/create", post(encounter::handler::create))
        .route("/api/encounters", get(encounter::handler::get_all))
        .route("/api/encounter/{*id}", get(encounter::handler::get_by_id))
        .route("/api/encounter/{*id}", put(encounter::handler::update))
        .fallback(initiative::index)
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:5173")
        .await
        .expect("failed to bind to port 8080");
    axum::serve(listener, app)
        .await
        .expect("failed to start server on port 5173");
}
