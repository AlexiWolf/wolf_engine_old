use wolf_engine::prelude::*;

pub fn main() {
    let mut engine = Engine::default();
    let mut number = 0;

    while let Some(event) = engine.next_event() {
        match event {
            Event::Quit => println!("Quit event recieved.  Goodbye!"),
            Event::Update => if number == 3 {
                engine.send_event(Event::Quit); 
            } else {
                number += 1;
            },
            Event::Render => {
                println!("{}: Hello, World!", number);
            }
            Event::EventsCleared => {
                engine.send_event(Event::Update);
                engine.send_event(Event::Render);
            }
        }
    }
}
