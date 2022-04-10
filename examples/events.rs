use log::*;
use rc_event_queue::LendingIterator;
use wolf_engine::contexts::EventContext;
use wolf_engine::event::EventReader;
use wolf_engine::utils::trust_cell::Ref;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    Engine::new().run(Box::from(ExampleState::new()));
}

pub struct ExampleState {
    number: usize,
    number_reader: Option<EventReader<usize>>,
}

impl State for ExampleState {
    fn setup(&mut self, context: &mut Context) {
        let event_context = EventContext::<usize>::default();
        self.number_reader = Some(event_context.reader());
        context.add(event_context).unwrap();
    }

    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        let events = Self::get_event_context(context);
        if self.number < 100 {
            self.number += 1;
            events.push(self.number);
            None
        } else {
            Some(Transition::Quit)
        }
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        if let Some(number_reader) = self.number_reader.as_mut() {
            while let Some(number) = number_reader.iter().next() {
                info!("{}", number);
            }
        }
    }
}

impl ExampleState {
    pub fn new() -> Self {
        Self {
            number: 0,
            number_reader: None,
        }
    }

    pub fn get_event_context(context: &Context) -> Ref<EventContext<usize>> {
        context
            .borrow::<EventContext<usize>>()
            .expect("the context has no EventContext<usize>")
    }
}
