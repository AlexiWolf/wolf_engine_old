use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder};
use wolf_engine::prelude::*;

pub fn main() {
    env_logger::init();
    let winit_event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Wolf Engine - Graphics Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&winit_event_loop)
        .unwrap();
    let graphics = pollster::block_on(wolf_engine::graphics::init(
        GraphicsSettings::default(),
        Some(&window),
    ));

    let _ = winit_event_loop.run(|event, window_target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => window_target.exit(),
            _ => (),
        }
    });
}
