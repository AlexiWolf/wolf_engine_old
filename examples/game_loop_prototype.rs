use std::{thread, time::{Duration, Instant}};

use log::{LevelFilter, debug, info};
use winit::event_loop::{ControlFlow, EventLoop};
use wolf_engine::logging::logger;

fn main() {
    let logger = logger();
    logger.set_log_level(LevelFilter::Debug);
    let event_loop = EventLoop::new();
    let tick_rate: u64 = 120;
    let frame_rate: u64 = 30;
    let update_time = Duration::from_millis(1000 / tick_rate);
    let frame_time = Duration::from_millis(1000 / frame_rate);
    let mut previous = Instant::now();
    let mut last_update = Instant::now();
    let mut lag = Duration::from_secs(0);

    event_loop.run(
        move |_event, _, control_flow| {
            let current = Instant::now();
            previous = current;
            
            let elapsed = current - previous;
            lag += elapsed;
        
            while lag >= update_time {
                debug!("Update: {}", last_update.elapsed().as_millis());
                last_update = Instant::now();
                lag -= update_time;
            }
            *control_flow = ControlFlow::WaitUntil(Instant::now() + frame_time);
            info!("Rendering");
        }
    )

}
