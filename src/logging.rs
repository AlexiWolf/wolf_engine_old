use log::LevelFilter;

pub(crate) fn initialize_logging(level: LevelFilter) -> Result<Logger, ()> {
    Err(())
}

pub struct Logger;

impl Logger {
    pub fn add_log_target(&self, log_target: &dyn LogTarget) {}
}

pub trait LogTarget: 'static + Send + Sync {}

#[cfg(test)]
mod log_tests {
    use super::log_test_fixtures::*;
    use super::*;
    use log::info;

    #[test]
    fn should_log_to_connected_log_targets() {
        let logger =
            initialize_logging(LevelFilter::Trace).expect("Failed to initialize the logger");
        let log_target_a = TestLogTarget::new();
        let log_target_b = TestLogTarget::new();
        logger.add_log_target(&log_target_a);
        logger.add_log_target(&log_target_b);

        info!("Hello, World!");

        assert_eq!(
            log_target_a
                .latest_record()
                .expect("No message was sent")
                .args()
                .to_string(),
            "Hello, World!".to_string()
        );
        assert_eq!(
            log_target_b
                .latest_record()
                .expect("No message was sent")
                .args()
                .to_string(),
            "Hello, World!".to_string()
        );
    }
}

#[cfg(test)]
pub mod log_test_fixtures {
    use super::*;
    use log::Record;

    pub struct TestLogTarget;

    impl TestLogTarget {
        pub fn new() -> Self {
            Self
        }

        pub fn latest_record(&self) -> Option<Record> {
            None
        }
    }

    impl LogTarget for TestLogTarget {}
}
