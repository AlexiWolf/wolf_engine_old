use std::fmt::Display;

use log::{LevelFilter, debug, info};

use wolf_engine::{Context, ContextBuilder, Frames, GameLoop, LoopResult, Ticks, WolfEngineBuilder};


/// A very basic game loop for demonstration purposes.
/// 
/// This game loop works by just calling the `update` and `render` methods.  It also tracks some
/// information locally for display purposes.
pub struct CustomGameLoop {
    ticks: Ticks,
    frames: Frames
}

impl CustomGameLoop {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            frames: 0 
        }
    }

    fn track_tick_information(&mut self, context: &mut Context) {
        context.game_loop.add_tick(); // Updating the GameLoopContext is required.
        self.ticks += 1; // Optionally, you may track additional information.
    }

    fn track_frame_information(&mut self, context: &mut Context) {
        context.game_loop.add_frame(); // Updating the GameLoopContext is required.
        self.frames += 1; // Optionally, you may track additional information.
    }
}

impl GameLoop for CustomGameLoop {
    fn update<F>(&mut self, context: &mut Context, mut update_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult {
            self.track_tick_information(context);
            update_function(context);
    }

    fn render<F>(&mut self, context: &mut Context, mut render_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult {
            self.track_frame_information(context);
            render_function(context); 
            debug!("{}", &self);
   }
}

impl Display for CustomGameLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Sometimes game loops store information, and it can be helpful for debugging to display
        // that information.  This is optional, but encouraged.
        write!(f, "Custom Game Loop - {} ticks, {} frames", self.ticks, self.frames)
    }
}

pub fn main() { 
    // Logging will be initialized by the Context in the future.
    wolf_engine::logging::logger()
        .set_log_level(LevelFilter::Debug); 
    let custom_game_loop = CustomGameLoop::new();
    let context = ContextBuilder::new()
        .build();
    WolfEngineBuilder::with_custom_game_loop(custom_game_loop)
        .build(context)
        .run(
        |_context|{
            info!("Called the update function!");
        },
        |_context| {
            info!("Called the render function!")
        }
    );
}