use wolf_engine::prelude::*;

pub fn main() {
    let mut engine = Engine::default();
    let mut number = 0;

    while let Some(event) = engine.next_event() {
        match event {
            // Shut down the game.
            Event::Quit => println!("Quit event received.  Goodbye!"),
            // Update the game.
            Event::Update => {
                if number == 3 {
                    // To shut down the Engine, you must send a quit event.
                    engine.quit();
                } else {
                    number += 1;
                }
            }
            Event::Render => {
                // Render the game.
                println!("{}: Hello, World!", number);
            }
            Event::EventsCleared => {
                // Note: The engine will not emit Update / Render events on it's own.
                //       You are expected to do this yourself.
                engine.update();
                engine.render();
            }
        }
    }
}
