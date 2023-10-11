use wolf_engine::prelude::*;

pub struct Message(&'static str);

#[wolf_engine::ecs::system]
fn log_message(#[resource] message: &Message) {
    log::info!("{}", message.0);
}

pub fn main() {
    let (mut event_loop, mut context) = wolf_engine::init()
        .with_resources(|resources| {
            resources.add_resource(Message("Hello, World!"));
        })
        .with_update_schedule(|schedule| {
            schedule.add_thread_local(log_message_system()); 
        })
        .build();

    while let Some(event) = event_loop.next_event() {
        process_event(event, &mut context);
    }
}

pub fn process_event(event: Event<()>, context: &mut Context<()>) {
    match event {
        Event::EventsCleared => {
            context.update();
            context.quit();
        }
        Event::Quit => println!("Quit event received.  Goodbye!"),
        _ => (),
    }
}
