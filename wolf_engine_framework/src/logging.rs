//! Provides a default logging implementation using the [simple_logger] crate.

use log::LevelFilter;
use simple_logger::SimpleLogger;

/// Initializes the logging system with a pre-configured [SimpleLogger] instance.
///
/// This function is provided for those who don't need a complicated logging setup.  
/// Messages will be logged to the terminal.
///
/// # Examples
///
/// To use the default logger, just initialize it by calling this function and providing
/// it with the desired [LevelFilter].
///
/// ```
/// # use log::LevelFilter;
/// #
/// wolf_engine::logging::initialize_logging(LevelFilter::Debug);
/// ```
///
/// Messages are logged using [log] macros.
///
/// ```
/// # use log::info;
/// #
/// info!("Hello, world!");
/// ```
pub fn initialize_logging(log_level: LevelFilter) {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log_level)
        .with_utc_timestamps()
        .init()
        .expect("Failed to initialize the logger");
}
