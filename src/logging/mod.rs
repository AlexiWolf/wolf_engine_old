//! A small logging framework providing a [Logger], and easy integration.
//!
//! # Usage
//!
//! Initialization is handled automatically, but some additional options are exposed by the
//! [Logger].  Logging messages is done using the [log] crate macros.

mod logger;
mod log_target;

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::{LevelFilter, Log};

pub use logger::*;
pub use log_target::*;

/// Get a reference to the global Logger instance.
pub fn logger() -> &'static Logger {
    &*LOGGER
}

lazy_static! {
    /// A static reference to the global Logger.  This automatically sets up the logging framework.
    pub(crate) static ref LOGGER: &'static Logger = initialize_logging();
}

fn initialize_logging() -> &'static Logger {
    lazy_static!{
        static ref TEMPORARY_LOGGER: Logger = Logger::new();
    }
    let logger = &*TEMPORARY_LOGGER;
    log::set_logger(logger as &dyn Log)
        .expect("Failed to set the logger");
    log::set_max_level(LevelFilter::Info);
    logger
}
