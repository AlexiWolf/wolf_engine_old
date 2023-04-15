//! Provides a default logging implementation using [`SimpleLogger`].

use simple_logger::SimpleLogger;

/// Indicates the verbosity of the log system.
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        match self {
            Self::Trace => log::LevelFilter::Trace,
            Self::Debug => log::LevelFilter::Debug,
            Self::Info => log::LevelFilter::Info,
            Self::Warn => log::LevelFilter::Warn,
            Self::Error => log::LevelFilter::Error,
            Self::Off => log::LevelFilter::Off,
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

