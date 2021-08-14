use log::{LevelFilter, Log, Metadata, Record};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref LOGGER: Logger = Logger::new();
}

pub(crate) fn initialize_logging(level: LevelFilter) -> Result<&'static Logger, ()> {
    log::set_logger(&*LOGGER as &dyn Log)
        .map(|()| log::set_max_level(level))
        .expect("Failed to set the logger");
    Ok(&LOGGER)
}

pub struct Logger {
    log_targets: Arc<Mutex<Vec<&'static dyn LogTarget>>>
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_log_target(&self, log_target: &'static dyn LogTarget) {
        self.log_targets
            .lock()
            .unwrap()
            .push(log_target);
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        self.log_targets
            .lock()
            .unwrap()
            .iter()
            .for_each(|target| {
                target.log(record);
            });
    }

    fn flush(&self) {}
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            log_targets: Arc::new(Mutex::new(vec![]))
        }
    }
}


pub trait LogTarget: Send + Sync {
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
        let logger =
            initialize_logging(LevelFilter::Trace).expect("Failed to initialize the logger");
        logger.add_log_target(&*LOG_TARGET_A as &dyn LogTarget);
        logger.add_log_target(&*LOG_TARGET_B as &dyn LogTarget);

        info!("Hello, World!");

        assert_eq!(
            LOG_TARGET_B.last_message(),
            "Hello, World!".to_string()
        );
        assert_eq!(
            LOG_TARGET_B.last_message(),
            "Hello, World!".to_string()
        );
    }
}

#[cfg(test)]
pub mod log_test_fixtures {
    use super::*;
    use log::Record;

    pub struct TestLogTarget {
        pub records: Arc<Mutex<Vec<String>>>
    }

    impl TestLogTarget {
        pub fn new() -> Self {
            Self {
                records: Arc::new(Mutex::new(vec![]))
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
            self.records
                .lock()
                .unwrap()
                .push(message);
        }
    }
    unsafe impl Send for TestLogTarget {}
    unsafe impl Sync for TestLogTarget {}
}
