use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn initialize_logging(log_level: LevelFilter) {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log_level)
        .init()
        .expect("Failed to initialize the logger");
}
