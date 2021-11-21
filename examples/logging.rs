use log::{debug, error, info, warn, LevelFilter};
use wolf_engine::logging;

pub fn main() {
    logging::initialize_logging(LevelFilter::Debug);
    info!("Hello, world!");
    debug!("This is some debug information.");
    warn!("Here is a warning!");
    error!("Something has gone wrong!")
}
