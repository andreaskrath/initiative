use axum::{
    Router,
    routing::{delete, get, post, put},
};
use initiative::{encounter, monster, spell};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    async_main();
}

#[tokio::main]
async fn async_main() {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "initiative=info,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Initiative tracker backend");

    let db_connection_string = env::var("DB_CONNECTION_STRING")
        .expect("DB_CONNECTION_STRING environment variable is not set");

    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_string)
        .await
        .expect("failed to connect to database");
    info!("Database connection established");

    info!("Running database migrations...");
    sqlx::migrate!().run(&pool).await.unwrap();
    info!("Database migrations completed");

    let app = Router::new()
        .route("/assets/{*path}", get(initiative::assets))
        .route("/images/{*path}", get(initiative::images))
        .route("/api/spells/create", post(spell::handler::create))
        .route("/api/spells", get(spell::handler::get))
        .route("/api/spell/{*id}", get(spell::handler::get_by_id))
        .route("/api/spell/{*id}", delete(spell::handler::delete))
        .route("/api/monsters/create", post(monster::handler::create))
        .route("/api/monsters", get(monster::handler::get))
        .route("/api/monster/{*id}", get(monster::handler::get_by_id))
        .route("/api/monster/{*id}", delete(monster::handler::delete))
        .route("/api/encounters/create", post(encounter::handler::create))
        .route("/api/encounters", get(encounter::handler::get_all))
        .route("/api/encounter/{*id}", get(encounter::handler::get_by_id))
        .route("/api/encounter/{*id}", put(encounter::handler::update))
        .route("/api/encounter/{*id}", delete(encounter::handler::delete))
        .fallback(initiative::index)
        .with_state(pool);

    let addr = "127.0.0.1:5173";
    info!("Server listening on http://{}", addr);
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to port 5173");
    axum::serve(listener, app)
        .await
        .expect("failed to start server on port 5173");
}
