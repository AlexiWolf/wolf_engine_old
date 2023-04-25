use wolf_engine::logging::*;

pub fn main() {
    initialize_logging(LogLevel::Info);
    
    log::info!("Hello, world!");
}
