
/// Provides access to all engine state.
/// 
/// # Examples
/// 
/// Using the default game loop.
/// 
/// ```
/// # use wolf_engine::WolfEngine;
/// # use wolf_engine::FixedUpdateGameLoop
/// 
/// let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngine::with_default_game_loop()
///     .build();
/// ```
pub struct WolfEngine;

pub struct WolfEngineBuilder;

impl WolfEngineBuilder {
    pub fn with_default_game_loop() -> Self {
        Self
    }
}