use wolf_engine::prelude::*;

pub struct Message(&'static str);

#[wolf_engine::ecs::system]
fn log_message(#[resource] message: &Message) {
    log::info!("{}", message.0);
}

#[wolf_engine::ecs::system]
fn quit_after_3_updates(
    #[state] updates: &mut u32,
    #[resource] event_sender: &MainEventSender<()>,
) {
    if *updates == 3 {
        event_sender.send_event(Event::Quit).ok();
    } else {
        *updates += 1;
    }
}

pub fn main() {
    logging::initialize_logging(logging::LogLevel::Info);

    let (mut event_loop, mut context) = wolf_engine::framework::init()
        .with_resource(Message("Hello, World!"))
        .build();

    let mut schedule = Schedule::builder()
        .add_system(log_message_system())
        .add_system(quit_after_3_updates_system(1))
        .build();

    while let Some(event) = event_loop.next_event() {
        process_event(event, &mut context, &mut schedule);
    }
}

pub fn process_event(event: Event<()>, context: &mut Context<()>, schedule: &mut Schedule) {
    match event {
        Event::EventsCleared => {
            context.run_schedule(schedule);
        }
        Event::Quit => log::info!("Quit event received.  Goodbye!"),
        _ => (),
    }
}
