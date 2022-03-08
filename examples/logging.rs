use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    info!("Hello, world!");
    debug!("This is some debug information.");
    warn!("Here is a warning!");
    error!("Something has gone wrong!")
}
