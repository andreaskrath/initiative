use crate::Error;
use crate::repositories::Repository;
use crate::repositories::options::OptionsRepository;
use crate::repositories::options::Variant;

use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteSynchronous;
use std::path::PathBuf;

pub type Pool = SqlitePool;

pub async fn connect(path: PathBuf) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .create_if_missing(true)
        .synchronous(SqliteSynchronous::Normal);

    let result = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await;

    match result {
        Ok(pool) => {
            if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
                tracing::error!("failed to run migrations: {err:?}");

                return Err(Error::Connection);
            }

            Ok(pool)
        }
        Err(err) => {
            tracing::error!("failed to connect local client to '{path:?}': {err:?}");

            Err(Error::Connection)
        }
    }
}

pub struct Local {
    pool: SqlitePool,
}

impl Local {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl Repository for Local {}

#[async_trait::async_trait]
impl OptionsRepository for Local {
    async fn list_options(&self, variant: Variant) -> Result<Box<[String]>, Error> {
        let query = r#"
            SELECT value
            FROM options
            WHERE variant = $1
            ORDER BY sort_order;
        "#;

        let options = sqlx::query_scalar(query)
            .bind(variant)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("fetched {} '{:?}' options", options.len(), variant);

        Ok(options.into_boxed_slice())
    }
}
