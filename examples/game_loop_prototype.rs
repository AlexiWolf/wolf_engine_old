use std::{
    fmt::Display,
    thread,
    time::{Duration, Instant},
};

use log::*;
use wolf_engine::logging::logger;

fn main() {
    initialize_logging();
    let mut game_loop = GameLoop::new(120.0, Duration::from_millis(100));
    game_loop.run(custom_update_function, custom_render_function);
}

fn custom_update_function(game_loop: &GameLoop) {
    debug!("Update : {}", game_loop);
}

fn custom_render_function(game_loop: &GameLoop) {
    info!(" Render : {}", game_loop);
    limit_fps(30.0);
}

fn limit_fps(fps: f64) {
    thread::sleep(Duration::from_secs_f64(1.0 / fps));
}

fn initialize_logging() {
    let logger = logger();
    logger.set_log_level(LevelFilter::Debug);
}

// #[derive(Debug)]
pub struct GameLoop {
    lag: Duration,
    previous_update: Instant,
    current_update: Instant,
    update_rate: f64,
    update_step: Duration,
    updates: u64,
    max_update_time: Duration,
    real_update_time: Duration,
    real_update_time_this_frame: Duration,
    previous_frame: Instant,
    current_frame: Instant,
    frame_time: Duration,
    frames: u64,
}

impl GameLoop {
    pub fn new(update_rate: f64, max_update_time: Duration) -> Self {
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        let update_step = Duration::from_secs_f64(1.0 / update_rate);
        Self {
            lag: zero,
            previous_update: now,
            current_update: now,
            update_rate,
            update_step,
            updates: 0,
            max_update_time,
            real_update_time: zero,
            real_update_time_this_frame: zero,
            previous_frame: now,
            current_frame: now,
            frame_time: zero,
            frames: 0,
        }
    }

    pub fn run(&mut self, update_function: fn(&Self), render_function: fn(&Self)) {
        loop {
            self.update(update_function);
            self.render(render_function);
        }
    }

    pub fn update(&mut self, update_function: fn(&Self)) {
        self.calculate_lag();
        self.real_update_time_this_frame = Duration::from_secs(0);
        while self.can_update() {
            let start_time = Instant::now();
            update_function(&self);
            let stop_time = Instant::now();
            self.real_update_time = stop_time - start_time;
            self.real_update_time_this_frame += self.real_update_time;
            self.lag -= self.update_step;
            self.updates += 1;
        }
        self.log_exceeded_update_limit();
    }

    fn calculate_lag(&mut self) {
        self.current_update = Instant::now();
        let elapsed_time = self.current_update - self.previous_update;
        self.previous_update = self.current_update;
        self.lag += elapsed_time;
    }

    pub fn render(&mut self, render_function: fn(&Self)) {
        self.calculate_frame_time();
        render_function(&self);
        self.frames += 1;
    }

    fn calculate_frame_time(&mut self) {
        self.current_frame = Instant::now();
        self.frame_time = self.current_frame - self.previous_frame;
        self.previous_frame = self.current_frame;
    }

    pub fn fps(&self) -> u64 {
        if self.frames > 1 {
            (1.0 / self.frame_time.as_secs_f64()).ceil() as u64
        } else {
            0
        }
    }
}

impl GameLoop {
    fn can_update(&self) -> bool {
        !self.has_exceeded_update_time_limit() && self.is_lag_greater_than_update_step()
    }

    fn log_exceeded_update_limit(&self) {
        if self.has_exceeded_update_time_limit() {
            debug!("Update limit exceeded!");
        }
    }

    fn is_lag_greater_than_update_step(&self) -> bool {
        self.lag >= self.update_step
    }

    fn has_exceeded_update_time_limit(&self) -> bool {
        self.real_update_time_this_frame > self.max_update_time
    }
}

impl Display for GameLoop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Frames: {}, {}fps ({}ms) - Updates: {}, {}ups, simulated {}ms in {}us ({}us total), Lag: {}ms",
            self.frames,
            self.fps(),
            self.frame_time.as_millis(),
            self.updates,
            self.update_rate,
            self.update_step.as_millis(),
            self.real_update_time.as_micros(),
            self.real_update_time_this_frame.as_micros(),
            self.lag.as_millis(),
        )
    }
}
