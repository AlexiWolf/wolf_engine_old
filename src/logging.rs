//! Provides a default logging implementation using the [simple_logger] crate.

use log::LevelFilter;
use simple_logger::SimpleLogger;

/// Initialize the default logger.
/// 
/// This function is provided for those who don't need a complicated logging setup.  The default
/// logger will output messages to the terminal.
/// 
/// # Examples
/// 
/// To use the default logger, just initialize it by calling this function.
/// 
/// ```
/// # use wolf_engine::logging;
/// # use log::LevelFilter;
/// #
/// logging::initialize_logging(LevelFilter::Debug);
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
        .init()
        .expect("Failed to initialize the logger");
}
