use log::*;
use wolf_engine::*;

pub fn main() {
    // If the "logging" feature is enabled, Wolf Engine includes a default logger for
    // convenience, but using it is optional.  Feel free to bring your own logger instead.
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    // Initialize your game state.
    let game = FizzBuzzState::new();

    // Pass your state to the engine.  Have fun!
    Engine::new().run(Box::from(game));
}

// Your game is implemented as one or many game states.  A game state stores all the
// data and logic for your game.  You pass an instance of the game state to the engine,
// and the engine will run the game state.  Game states can interact with the engine
// through the Context object, and by returning Transitions.
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
            Some(Transition::Clean) // Tell the engine we want to quit.
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
