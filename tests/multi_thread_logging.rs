#[cfg(test)]
#[cfg(feature = "logging")]
pub mod multi_threaded_logging_tests {
    use log::*;
    use std::thread;
    use wolf_engine::*;
    use wolf_engine::logging::*;

    #[test]
    pub fn should_not_panic_in_multi_threaded_environment() {
        logging::initialize_logging(LogLevel::Info);

        let thread = thread::spawn(|| {
            info!("Hello, world!");
        });
        thread.join().unwrap();
    }
}
