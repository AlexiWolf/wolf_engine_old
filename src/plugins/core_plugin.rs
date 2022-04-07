use crate::contexts::{EventContext, SchedulerContext};
use crate::event::Event;
use crate::*;

/// Provides core functionality that **must** be loaded in order for the engine to work.
pub(crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        Ok(engine_builder
            .with_engine_core(Box::from(run_while_has_active_state))
            .with_subcontext(SchedulerContext::new())
            .with_subcontext(EventContext::<Event>::default()))
    }
}
