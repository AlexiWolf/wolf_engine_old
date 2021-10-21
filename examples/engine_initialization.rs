use std::{sync::Arc, thread, time::Duration};

use log::info;
use wolf_engine::WolfEngineBuilder;

pub fn main() {
    let wolf_engine = WolfEngineBuilder::with_default_game_loop().build();
    let _logging = wolf_engine::logging::logger();

    let message = Arc::from(std::sync::Mutex::from("Hello, World!".to_string()));
    let number = Arc::from(std::sync::Mutex::from(0 as u64));

    wolf_engine.run(
        |_| {
            let mut message = message.lock().unwrap();
            let mut number = number.lock().unwrap();

            *number += 1;
            if *number % 2 == 0 {
                *message = "Hello, World!".to_string();
            } else {
                *message = "Hello, from Wolf Engine!".to_string();
            }
        },
        |_| {
            info!(
                "{} - {} Ticks",
                message.lock().unwrap(),
                number.lock().unwrap()
            );
            thread::sleep(Duration::from_secs_f64(1.0));
        },
    );
}
