use log::info;
use wolf_engine::logging;

pub fn main() {
    logging::initialize_logging();
    info!("Hello, world!");
}