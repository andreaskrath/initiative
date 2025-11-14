use std::str::FromStr;
use thiserror::Error;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

/// The default log level if not specified elsewhere.
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::INFO;

#[derive(Debug, Error)]
enum LoggingError {
    #[error("failed to parse '{0}', reason: {1}")]
    FailedEnvironmentVariableParsing(String, String),
}

/// Initialize a subscriber for tracing events.
pub fn init() {
    let level = level_filter();

    let (log_level, init_error) = match level {
        Ok(Some(log_level)) => (log_level, None),
        Ok(None) => (DEFAULT_LOG_LEVEL, None),
        Err(err) => (DEFAULT_LOG_LEVEL, Some(err)),
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_ansi(true)
        .with_level(true)
        .try_init()
        .expect("failed to initiative subscriber");

    if let Some(err) = init_error {
        error!("could not parse log filter from environment: {}", err);
    }

    info!("logging with max level '{}'", log_level);
}

/// There are three possible return values from this function:
/// - `Ok(None)` - no log level specified in environment variable
/// - `Ok(Some)` - a correct log level is specified in environment variable and contained in `Some`
/// - `Err` - an incorrect log level is specified in environment variable, and could not be parsed
fn level_filter() -> Result<Option<LevelFilter>, LoggingError> {
    let env_initiative_log = option_env!("INITIATIVE_LOG");
    let Some(env_log_level) = env_initiative_log else {
        return Ok(None);
    };

    match LevelFilter::from_str(env_log_level) {
        Ok(log_level) => Ok(Some(log_level)),
        Err(err) => Err(LoggingError::FailedEnvironmentVariableParsing(
            String::from(env_log_level),
            err.to_string(),
        )),
    }
}
