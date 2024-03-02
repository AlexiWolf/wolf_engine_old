use wolf_engine::prelude::*;

pub fn main() {
    env_logger::init();
    let graphics_settings = GraphicsSettings::default();
    let graphics = wolf_engine::graphics::init();
}
