use log::*;
pub fn main() {
    #[cfg(feature = "logging")]
    wolf_engine::initialize_logging(LevelFilter::Debug);

    info!("Hello, world!");
    debug!("This is some debug information.");
    warn!("Here is a warning!");
    error!("Something has gone wrong!")
}
