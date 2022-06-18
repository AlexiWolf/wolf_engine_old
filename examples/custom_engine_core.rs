use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Info);

    EngineBuilder::new()
        .with_engine_core(Box::from(my_custom_core_function))
        .build()
        .expect("Failed to build the engine")
        .run(Box::from(EmptyState));
}

/// Engine cores are just normal functions.
///
/// The only requirements are they take an [Engine] as an argument, and they don't
/// return anything.
///
/// Core functions offer you a lot of control because they give you full ownership over
/// the running [Engine] instance.  This essentially means you can directly control over
/// how the engine runs things.
pub fn my_custom_core_function(mut engine: Engine) -> Engine {
    info!("Hello, from a custom core function!");

    // Lets make a simple core that runs 10 times, the quits.
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
