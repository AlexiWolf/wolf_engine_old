use log::*;
use wolf_engine::*;

pub fn main() {
    // Wolf Engine includes a default logger for convenience, but using it is optional.
    // Feel free to bring your own logger.
    initialize_logging(LevelFilter::Debug);

    // Start by initializing the Context object.
    let (context, event_loop) = ContextBuilder::new()
        // Custom settings go here.
        .build();

    // Then build an instance of the engine.
    let engine = WolfEngineBuilder::with_default_game_loop()
        // Custom settings go here.
        .build(context);

    // Initialize your game state.
    let game = FizzBuzzState::new();

    // Then pass your game state to the engine on startup.  Have fun!
    engine.run(Box::from(game), event_loop);
}

pub struct FizzBuzzState {
    number: u64,
}

impl FizzBuzzState {
    pub fn new() -> Self {
        Self { number: 0 }
    }

    fn fizz_buzz(number: u64) -> String {
        if number % 15 == 0 {
            "fizz-buzz".to_string()
        } else if number % 5 == 0 {
            "buzz".to_string()
        } else if number % 3 == 0 {
            "fizz".to_string()
        } else {
            number.to_string()
        }
    }
}

impl State for FizzBuzzState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.number == 100 {
            info!("Goodbye!");
            Some(Transition::Quit) // Tell the engine we want to quit.
        } else {
            self.number += 1;
            info!("{}", Self::fizz_buzz(self.number));
            None // Tell the engine we want to continue running this state.
        }
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        // Nothing to render for this example.
    }
}
