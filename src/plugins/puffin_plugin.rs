use crate::*;

/// Provides profiling using [puffin].
pub struct PuffinPlugin;

impl Plugin for PuffinPlugin {
    fn setup(&mut self,engine_builder:EngineBuilder) -> PluginResult {
        if cfg!(feature = "profiling") {
            puffin::set_scopes_on(true);
        }
        if cfg!(feature = "http_profiling") {
            use contexts::PuffinHttpContext;
            let server_address = "0.0.0.0:8585";
            let puffin_server_result = PuffinHttpContext::new(server_address);
        }
        Ok(engine_builder)
    }
}
