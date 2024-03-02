use raw_window_handle::HasWindowHandle;

pub fn init(settings: GraphicsSettings, window_handle: Option<&dyn HasWindowHandle>) -> GraphicsContext {
    todo!()
}

pub struct GraphicsContext { 

}

#[derive(Default)]
pub struct GraphicsSettings {

}

pub mod prelude {
    pub use super::GraphicsSettings;
}
