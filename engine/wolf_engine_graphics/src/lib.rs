use raw_window_handle::HasWindowHandle;

pub async fn init(
    settings: GraphicsSettings,
    window_handle: Option<&dyn HasWindowHandle>,
) -> GraphicsContext {
    GraphicsContext {

    }
}

pub struct GraphicsContext<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Option<wgpu::Surface<'a>>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    surface_size: Option<(u32, u32)>,
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub mod prelude {
    pub use super::GraphicsSettings;
}
