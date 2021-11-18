use log::info;
use wolf_engine::logging;

pub fn main() {
    // Wolf Engine provides terminal logging through the SimpleLogger crate.
    logging::initialize_logging();
    info!("Hello, world!");
}