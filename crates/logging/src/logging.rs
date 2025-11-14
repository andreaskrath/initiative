use tracing_subscriber::EnvFilter;

/// Initialize a subscriber for tracing events.
///
/// Reads the `RUST_LOG` environment variable to configure per-crate log levels.
/// If `RUST_LOG` is not set, defaults to `info` level for all crates.
///
/// Example `RUST_LOG` values:
/// - `debug` - all crates log at debug level
/// - `error,my_crate=debug` - all crates log errors, my_crate logs debug
/// - `error,my_crate=debug,other_crate=trace` - multiple overrides
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_ansi(true)
        .with_level(true)
        .init();
}
