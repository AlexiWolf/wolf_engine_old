use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}};
use wolf_engine::prelude::*;

pub fn main() {
    env_logger::init();
    let winit_event_loop = EventLoop::new().unwrap();
    let mut window = None;
    let graphics = pollster::block_on(wolf_engine::graphics::init(
        GraphicsSettings::default(),
        None,
    ));

    let _ = winit_event_loop.run(|event, window_target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => window_target.exit(),
            Event::Resumed => {
                window = Some(WindowBuilder::new()
                    .with_title("Wolf Engine - Graphics Example")
                    .with_inner_size(PhysicalSize::new(800, 600))
                    .build(&window_target)
                    .unwrap()
                );
            },
            _ => (),
        }
    });
}
