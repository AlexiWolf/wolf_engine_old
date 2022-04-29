use crate::{*, contexts::*};

/// Provides profiling using [puffin].
pub struct PuffinPlugin;

impl Plugin for PuffinPlugin {
    fn setup(&mut self,engine_builder:EngineBuilder) -> PluginResult {
        if cfg!(feature = "profiling") {
            puffin::set_scopes_on(true);
        }
        if cfg!(feature = "http_profiling") {
            let puffin_server_result = PuffinHttpContext::new();
        }
        Ok(engine_builder)
    }
}
