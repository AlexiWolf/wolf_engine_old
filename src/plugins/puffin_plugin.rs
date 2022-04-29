use crate::*;

/// Provides profiling using [puffin].
pub struct PuffinPlugin;

impl Plugin for PuffinPlugin {
    #[allow(unused_mut)]
    fn setup(&mut self, mut engine_builder: EngineBuilder) -> PluginResult {
        if cfg!(feature = "profiling") {
            puffin::set_scopes_on(true);
        }
        #[cfg(feature = "http_profiling")]
        {
            engine_builder = enable_puffin_http(engine_builder);
        }
        Ok(engine_builder)
    }
}

#[cfg(feature = "http_profiling")]
fn enable_puffin_http(mut engine_builder: EngineBuilder) -> EngineBuilder {
    let puffin_server_result = contexts::PuffinHttpContext::new();
    if puffin_server_result.is_ok() {
        let http_context = puffin_server_result.unwrap();
        log::info!("Successfully created Puffin HTTP server at: 0.0.0.0:8585");
        engine_builder = engine_builder.with_subcontext(http_context);
    } else {
        log::warn!(
            "Failed to create Puffin HTTP server at: 0.0.0.0:8585: {}",
            puffin_server_result.err().unwrap()
        );
    }
    engine_builder
}
