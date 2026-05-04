mod options;
mod spells;

use crate::Error;
use crate::repositories::Repository;

use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteSynchronous;
use std::path::PathBuf;

pub type Pool = SqlitePool;

#[derive(Debug, Clone)]
pub struct Local {
    pool: SqlitePool,
    images_dir: PathBuf,
}

impl Local {
    pub async fn new(path: PathBuf) -> Result<Self, Error> {
        let pool = connect(path).await?;
        let images_dir = dirs::data_local_dir()
            .expect("failed to get data dir")
            .join("images");

        // Create the images dir, to ensure it exists for when images are inserted.
        //
        // TODO: Maybe move this to some async stuff, but it might not matter.
        std::fs::create_dir_all(&images_dir).expect("failed to create images directory");

        Ok(Self { pool, images_dir })
    }
}

impl Repository for Local {}

async fn connect(path: PathBuf) -> Result<SqlitePool, Error> {
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
