use wolf_engine::prelude::*;

pub struct Message(&'static str);

#[wolf_engine::ecs::system]
fn log_message(#[resource] message: &Message) {
    log::info!("{}", message.0);
}

#[wolf_engine::ecs::system]
fn quit_after_3_updates(
    #[state] updates: &mut u32,
    #[resource] event_sender: &EngineEventSender<()>,
) {
    if *updates == 3 {
        event_sender.send_event(Event::Quit).ok();
    } else {
        *updates += 1;
    }
}

pub fn main() {
    logging::initialize_logging(logging::LogLevel::Info);

    let (mut event_loop, mut context) = wolf_engine::init()
        .with_resources(|resources| {
            resources.add_resource(Message("Hello, World!"));
        })
        .with_update_schedule(|schedule| {
            schedule
                .add_thread_local(log_message_system())
                .add_thread_local(quit_after_3_updates_system(1));
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
        }
        Event::Quit => log::info!("Quit event received.  Goodbye!"),
        _ => (),
    }
}
