use wolf_engine::prelude::*;

pub fn main() {
    let (mut event_loop, mut context) = wolf_engine::init().build();

    while let Some(event) = event_loop.next_event() {
        process_event(event, &mut context);
    }
}

pub fn process_event(event: Event<()>, context: &mut Context<()>) {
    match event {
        Event::EventsCleared => {
            context.quit();
        }
        Event::Quit => println!("Quit event received.  Goodbye!"),
        _ => (),
    }
}
