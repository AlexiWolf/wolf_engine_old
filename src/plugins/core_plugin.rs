use crate::contexts::{EventContext, SchedulerContext};
use crate::event::Event;
use crate::*;

pub(crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder {
        engine_builder
            .with_subcontext(SchedulerContext::new())
            .with_subcontext(EventContext::<Event>::default())
    }
}
