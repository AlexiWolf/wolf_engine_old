use wolf_engine::prelude::*;

pub fn main() {
    env_logger::init();
    let graphics_settings = GraphicsSettings::default();
    let window = None;
    let graphics = pollster::block_on(wolf_engine::graphics::init(graphics_settings, window));
}
