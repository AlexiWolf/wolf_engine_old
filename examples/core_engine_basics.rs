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

pub fn process_event(event: Event<()>, context: &mut Context<GameData, ()>) {
    match event {
        Event::EventsCleared => {
            update(context);
            display(context);
        }
        Event::Quit => println!("Quit event received.  Goodbye!"),
        _ => (),
    }
}

pub fn update(context: &mut Context<GameData, ()>) {
    if context.data.number == 3 {
        context.quit();
    } else {
        context.data.number += 1;
    }
}

pub fn display(context: &mut Context<GameData, ()>) {
    println!("{}", context.data.number);
}
