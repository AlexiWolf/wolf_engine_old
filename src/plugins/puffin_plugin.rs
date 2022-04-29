use crate::*;

/// Provides profiling using [puffin].
pub struct PuffinPlugin;

impl Plugin for PuffinPlugin {
    fn setup(&mut self,engine_builder:EngineBuilder) -> PluginResult {
        Ok(engine_builder)
    }

}
