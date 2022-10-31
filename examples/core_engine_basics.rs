use wolf_engine::prelude::*;

pub fn main() {
    let mut engine = Engine::default();

    while let Some(event) = engine.next_event() {
        match event {
            Event::Quit => todo!(),
            Event::Update => todo!(),
            Event::Render => todo!(),
            Event::EventsCleared => todo!(),
        }
    }
}
