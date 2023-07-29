use wolf_engine::prelude::*;

pub struct GameData {
    pub number: i32,
}

pub fn main() {
    let (mut event_loop, mut context) = wolf_engine::init(GameData { number: 0 });

    while let Some(event) = event_loop.next_event() {
        process_event(event, &mut context);
    }
}

pub fn process_event(event: Event, context: &mut Context<GameData>) {
    match event {
        // Shut down the game.
        Event::Quit => println!("Quit event received.  Goodbye!"),
        // Update the game.
        Event::Update => {
            if context.data.number == 3 {
                // To shut down the Engine, you must send a quit event.
                context.quit();
            } else {
                context.data.number += 1;
            }
        }
        Event::Render => println!("{}", context.data.number),
        Event::EventsCleared => {
            // Note: The engine will not emit Update / Render events on it's own.
            //       You are expected to do this yourself.
            context.update();
            context.render();
        }
        _ => (),
    }
}
