use winit::{window::WindowBuilder, event_loop::EventLoop, dpi::PhysicalSize};
use wolf_engine::prelude::*;

pub fn main() {
    env_logger::init();
    let winit_event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Wolf Engine - Graphics Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&winit_event_loop)
        .unwrap();
    let graphics_settings = GraphicsSettings::default();
    let graphics = pollster::block_on(wolf_engine::graphics::init(graphics_settings, Some(&window)));
}
