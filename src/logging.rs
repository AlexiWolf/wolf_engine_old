use log::LevelFilter;

pub(crate) fn initialize_logging(level: LevelFilter) -> Result<Logger, ()> {
    Err(())
}

pub struct Logger;

#[cfg(test)]
mod log_tests {
    use super::*;
    use super::log_test_fixtures::*;

    #[test]
    fn should_log_to_connected_log_targets() {
        let logger = initialize_logging(LevelFilter::Trace)
            .expect("Failed to initialize the logger");
        let test_log_target = TestLogTarget::new();
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
    }
}
