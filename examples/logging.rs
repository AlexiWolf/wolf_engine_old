use log::{debug, error, info, trace, warn, LevelFilter};
use wolf_engine::logging::*;

fn main() {
    let logger = logger();
    logger.set_log_level(LevelFilter::Trace);

    trace!("Trace information.");
    debug!("Debug information.");
    info!("Normal information.");
    warn!("Warning information");
    error!("Critical information.");
}
