use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Info);

    EngineBuilder::new()
        .with_engine_core(Box::from(MyMainLoop))
        .build()
        .expect("Failed to build the engine")
        .run(Box::from(EmptyState));
}

// A main loop is any struct which implements the `MainLoop` trait.
//
// Main loops offer you a lot of control because they give you full ownership over
// the running `Engine` instance.  This essentially means you can directly control
// how the engine runs things.
pub struct MyMainLoop;

impl MainLoop for MyMainLoop {
    fn run(&mut self, mut engine: Engine) -> Engine {
        info!("Hello, from a custom core function!");

        // Lets make a simple main loop that runs 10 times, the quits.
        let mut iterations = 0;

        while iterations > 0 {
            engine
                .state_stack
                .active_mut()
                .unwrap()
                .update(&mut engine.context);

            iterations += 1;
        }

        engine
    }
}
