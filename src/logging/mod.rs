//! A small logging framework providing a [Logger], and easy integration.
//!
//! # Usage
//!
//! Initialization is handled automatically, but some additional options are exposed by the
//! [Logger].  Logging messages is done using the [log] crate macros.

mod log_target;
mod logger;
mod terminal_log_target;

use lazy_static::lazy_static;
use log::{LevelFilter, Log};
use std::sync::{Arc, Mutex};

pub use log_target::*;
pub use logger::*;
pub use terminal_log_target::*;

/// Get a reference to the global Logger instance.
pub fn logger() -> &'static Logger {
    &*LOGGER
}

lazy_static! {
    /// A static reference to the global Logger.  This automatically sets up the logging framework.
    pub(crate) static ref LOGGER: &'static Logger = initialize_logging();
}

fn initialize_logging() -> &'static Logger {
    let logger = create_logger();
    initialize_log_framework(logger);
    add_default_log_targets(logger);
    logger
}

fn create_logger() -> &'static Logger {
    lazy_static! {
        static ref TEMPORARY_LOGGER: Logger = Logger::initialize();
    }
    let logger: &Logger = &*TEMPORARY_LOGGER;
    logger
}

fn initialize_log_framework(logger: &'static Logger) {
    log::set_logger(logger as &dyn Log).expect("Failed to set the logger");
    log::set_max_level(LevelFilter::Info);
}

fn add_default_log_targets(logger: &'static Logger) {
    logger.add_log_target(&TERMINAL_LOG_TARGET);
}
