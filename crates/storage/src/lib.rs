pub mod clients;
pub mod records;
pub mod models;
pub mod repositories;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error("already exists")]
    AlreadyExists,

    #[error("decode")]
    Decode,

    #[error("query")]
    Query,

    #[error("connection")]
    Connection,
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(err) if err.is_unique_violation() => Self::AlreadyExists,
            sqlx::Error::ColumnDecode { index, source } => {
                tracing::error!("column decode failure at '{index}': {source:?}");
                Self::Decode
            }
            sqlx::Error::Decode(source) => {
                tracing::error!("decode failure: {source:?}");
                Self::Decode
            }
            sqlx::Error::TypeNotFound { type_name } => {
                tracing::error!("type not found: '{type_name}'");
                Self::Decode
            }
            sqlx::Error::ColumnNotFound(name) => {
                tracing::error!("column not found: '{name}'");
                Self::Decode
            }
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                tracing::error!("column index {index} out of bounds (len: {len})");
                Self::Decode
            }

            // DB unreachable or broken
            sqlx::Error::Io(err) => {
                tracing::error!("io error: {err:?}");
                Self::Connection
            }
            sqlx::Error::Tls(err) => {
                tracing::error!("tls error: {err:?}");
                Self::Connection
            }
            sqlx::Error::Protocol(msg) => {
                tracing::error!("protocol error: {msg}");
                Self::Connection
            }
            sqlx::Error::Configuration(err) => {
                tracing::error!("configuration error: {err:?}");
                Self::Connection
            }
            sqlx::Error::PoolTimedOut => {
                tracing::error!("pool timed out");
                Self::Connection
            }
            sqlx::Error::PoolClosed => {
                tracing::error!("pool closed");
                Self::Connection
            }
            sqlx::Error::WorkerCrashed => {
                tracing::error!("worker crashed");
                Self::Connection
            }
            sqlx::Error::Migrate(err) => {
                tracing::error!("migration error: {err:?}");
                Self::Connection
            }
            sqlx::Error::BeginFailed => {
                tracing::error!("failed to begin transaction");
                Self::Connection
            }
            sqlx::Error::AnyDriverError(err) => {
                tracing::error!("driver error: {err:?}");
                Self::Connection
            }

            // Remaining database errors and bad queries
            err => {
                tracing::error!("query error: {err:?}");
                Self::Query
            }
        }
    }
}
