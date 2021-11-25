use log::{debug, error, info, warn, LevelFilter};

pub fn main() {
    wolf_engine::initialize_logging(LevelFilter::Debug);
    info!("Hello, world!");
    debug!("This is some debug information.");
    warn!("Here is a warning!");
    error!("Something has gone wrong!")
}
