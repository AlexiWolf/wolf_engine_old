use std::time::Duration;

use log::{LevelFilter, info};
use wolf_engine::{logging, ContextBuilder, WolfEngineBuilder};

fn main() {
    logging::initialize_logging(LevelFilter::Trace);
    let context = ContextBuilder::new()
        .build();
    let engine = WolfEngineBuilder::with_default_game_loop()
        .build(context);
    engine.run(
        |_| {
            std::thread::sleep(Duration::from_millis(4)); // Simulate a longer update time. 
            info!("Tick");            
        }, 
        |_| {
            std::thread::sleep(Duration::from_millis(16)); // Simulate 60 fps.
            info!("Render");
        }
    );
}
