//! Provides a default logging implementation using [`SimpleLogger`].

use simple_logger::SimpleLogger;

/// Indicates the verbosity of the log system.
pub enum LogLevel {
    /// Log all messages.
    Trace,

    /// Log debug messages.
    Debug,

    /// Log info messages.
    Info,

    /// Log warning messages.
    Warn,

    /// Log error messages.
    Error,

    /// Disable log messages.
    Off,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => log::LevelFilter::Trace,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Off => log::LevelFilter::Off,
        }
    }
}

/// Initializes the logging system with a pre-configured [SimpleLogger] instance.
///
/// This function is provided for those who don't need a complicated logging setup.  Messages will
/// be logged to the terminal.
///
/// # Examples
///
/// To use the default logger, just initialize it by calling this function and providing it with
/// the desired [LogLevel].
///
/// ```
/// # use wolf_engine_core::logging::LogLevel;
/// ```
///
/// Messages are logged using [log] macros.
///
/// ```
/// # use log::info;
/// #
/// info!("Hello, world!");
/// ```
pub fn initialize_logging(log_level: LogLevel) {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log_level.into())
        .with_utc_timestamps()
        .init()
        .expect("Failed to initialize the logger");
}
