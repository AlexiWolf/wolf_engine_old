use std::{thread, time::{Duration, Instant}};

use log::*;
use wolf_engine::logging::logger;

fn main() {
    initialize_logging();
    let mut time_context = TimeContext::new(120);
    loop {
        process_input();
        update(&mut time_context);
        render(30, &time_context);
    }
}

fn process_input() {
    trace!("input");
}

fn update(time_context: &mut TimeContext) {
    time_context.tick(|_time_context, time_step| {
        debug!("Update! Timestep: {}ms", time_step.as_millis());
    });
}
 
fn render(fps: u64, time_context: &TimeContext) {
    info!("render : {} updates", time_context.updates());
    thread::sleep(Duration::from_millis(1000 / fps));
}

fn initialize_logging() {
    let logger = logger();
    logger.set_log_level(LevelFilter::Debug);
}

#[derive(Debug)]
pub struct TimeContext {
    lag: Duration,
    previous_instant: Instant,
    current_instant: Instant,
    elapsed_time: Duration,
    updates: u64,
    ups: u64,
    update_step: Duration,
}

impl TimeContext {

    pub fn new(ups: u64) -> Self {
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        let update_step = Duration::from_millis(1000 / ups);
        Self {
            lag: zero,
            previous_instant: now,
            current_instant: now,
            elapsed_time: zero,
            updates: 0,
            ups,
            update_step
        }
    }

    fn tick(&mut self, update_function: fn(&Self, &Duration)) {
        self.current_instant = Instant::now();
        self.elapsed_time = self.current_instant - self.previous_instant;
        self.previous_instant = self.current_instant;
        self.lag += self.elapsed_time;

        while self.lag >= self.update_step {
            update_function(&self, &self.update_step);
            self.lag -= self.update_step;
            self.updates += 1;
        }
    }

    fn lag(&self) -> &Duration {
        &self.lag
    }

    fn previous_instant(&self) -> &Instant {
        &self.previous_instant
    }

    fn current_instant(&self) -> &Instant {
        &self.current_instant
    }

    fn elapsed_time(&self) -> &Duration {
        &self.elapsed_time
    }
    
    fn updates(&self) -> &u64 {
        &self.updates
    }
}
