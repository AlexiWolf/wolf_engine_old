use crate::logging::LogTarget;
use log::{Log, Metadata, Record, LevelFilter};
use std::sync::{Arc, Mutex};

/// Provides a [Log] that passes messages to all attached [LogTarget]s.
///
/// Logger doesn't do anything by itself.  Instead, it passes all messages to attached
/// [LogTarget]s.  If there are no targets, then log messages will be ignored.
///
/// A Logger cannot be created from the outside.  A global instance is created
/// automatically by the [logger](crate::logging::logger) function.  A
/// [TerminalLogTarget](crate::logging::TerminalLogTarget) is added automatically.
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
/// To get a reference to the global Logger, just use the [logger](crate::logging::logger) function.
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
    pub(crate) fn initialize() -> Self {
        Self {
            log_targets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn add_log_target(&self, log_target: &'static dyn LogTarget) {
        self.log_targets.lock().unwrap().push(log_target);
    }

    pub fn set_log_level(&self, log_level: LevelFilter) {
        log::set_max_level(log_level);
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

#[cfg(test)]
mod log_tests {
    use log::{info, trace};

    use crate::logging::log_target::LogTarget;
    use crate::logging::*;

    use super::log_test_fixtures::*;

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


    #[test]
    fn should_have_editable_log_level() {
        lazy_static! {
            static ref LOG_TARGET: TestLogTarget = TestLogTarget::new();
        }
        let log_target: &TestLogTarget = &*LOG_TARGET;
        let logger = logger();
        logger.add_log_target(log_target);

        info!("info");
        trace!("trace");

        assert_eq!(log_target.last_message(), "info".to_string());

        logger.set_log_level(LevelFilter::Trace);
        trace!("trace");

        assert_eq!(log_target.last_message(), "trace".to_string());
    }
}

#[doc(hidden)]
pub mod log_test_fixtures {
    use log::Record;

    use crate::logging::*;

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

    impl Default for TestLogTarget {
        fn default() -> Self {
            Self::new()
        }
    }

    unsafe impl Send for TestLogTarget {}
    unsafe impl Sync for TestLogTarget {}
}
