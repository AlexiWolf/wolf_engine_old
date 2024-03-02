use raw_window_handle::HasWindowHandle;

pub async fn init(
    settings: GraphicsSettings,
    window_handle: Option<&dyn HasWindowHandle>,
) -> GraphicsContext {
    GraphicsContext {

    }
}

pub struct GraphicsContext {}

#[derive(Default)]
pub struct GraphicsSettings {}

pub mod prelude {
    pub use super::GraphicsSettings;
}
