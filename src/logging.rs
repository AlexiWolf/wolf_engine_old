//! A small logging framework providing a [Logger], and easy integration.
//!
//! # Usage
//!
//! Initialization is handled automatically, but some additional options are exposed by the
//! [Logger].  Logging messages is done using the [log] crate macros.

use lazy_static::lazy_static;
use log::{LevelFilter, Log, Metadata, Record};
use std::sync::{Arc, Mutex};

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

/// Provides a [Log] that passes messages to all attached [LogTarget]s.
///
///
/// Logger doesn't do anything by itself.  Instead, it passes all messages to attached
/// [LogTarget]s.  If there are no targets, then log messages will be ignored.
///
/// A Logger cannot be created from the outside.  A global instance is created
/// automatically by the [logger] function.  You may get the Logger by calling [logger].
///
/// # Examples
///
/// Logging messages is done using the [log] crate macros.
///
/// ```
/// # use log::*;
/// info!("Hello, world!");
/// debug!("Have some debug information.");
/// error!("A problem has occurred!");
/// ```
///
/// To get a reference to the global Logger, just use the [logger] function.
///
/// ```
/// # use wolf_engine::logging;
/// let logger = logging::logger();
/// ```
///
/// Optionally, you can add your own [LogTarget]s.
///
/// ```
/// # use wolf_engine::logging::*;
/// # use wolf_engine::logging::log_test_fixtures::TestLogTarget;
/// # use lazy_static::lazy_static;
/// # let logger = logger();
/// # lazy_static! { static ref LOG_TARGET: TestLogTarget = TestLogTarget::new(); }
/// # let log_target = &*LOG_TARGET;
/// logger.add_log_target(log_target as &'static dyn LogTarget);
/// ```
pub struct Logger {
    log_targets: Arc<Mutex<Vec<&'static dyn LogTarget>>>,
}

impl Logger {
    fn new() -> Self {
        Self {
            log_targets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn add_log_target(&self, log_target: &'static dyn LogTarget) {
        self.log_targets.lock().unwrap().push(log_target);
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        self.log_targets.lock().unwrap().iter().for_each(|target| {
            target.log(record);
        });
    }

    fn flush(&self) {}
}

/// Allows easy integration with the [Logger].
pub trait LogTarget: Send + Sync {

    /// Process / display the log message.
    fn log(&self, record: &Record);
}

#[cfg(test)]
mod log_tests {
    use super::log_test_fixtures::*;
    use super::*;
    use log::info;

    #[test]
    fn should_log_to_connected_log_targets() {
        lazy_static! {
            static ref LOG_TARGET_A: TestLogTarget = TestLogTarget::new();
            static ref LOG_TARGET_B: TestLogTarget = TestLogTarget::new();
        }
        let logger = logger();
        logger.add_log_target(&*LOG_TARGET_A as &dyn LogTarget);
        logger.add_log_target(&*LOG_TARGET_B as &dyn LogTarget);

        info!("Hello, World!");

        assert_eq!(LOG_TARGET_B.last_message(), "Hello, World!".to_string());
        assert_eq!(LOG_TARGET_B.last_message(), "Hello, World!".to_string());
    }
}

#[doc(hidden)]
pub mod log_test_fixtures {
    use super::*;
    use log::Record;

    pub struct TestLogTarget {
        pub records: Arc<Mutex<Vec<String>>>,
    }

    impl TestLogTarget {
        pub fn new() -> Self {
            Self {
                records: Arc::new(Mutex::new(vec![])),
            }
        }

        pub fn last_message(&self) -> String {
            self.records
                .lock()
                .unwrap()
                .last()
                .expect("No messages have been sent")
                .clone()
        }
    }

    impl LogTarget for TestLogTarget {
        fn log(&self, record: &Record) {
            let message = record.args().to_string();
            self.records.lock().unwrap().push(message);
        }
    }

    unsafe impl Send for TestLogTarget {}
    unsafe impl Sync for TestLogTarget {}
}
