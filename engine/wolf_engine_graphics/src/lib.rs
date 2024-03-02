use raw_window_handle::HasWindowHandle;

pub async fn init(
    settings: GraphicsSettings,
    window: Option<&(dyn HasWindowHandle + Sync)>,
) -> GraphicsContext {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let surface = None;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false, 
        compatible_surface: if surface.is_some() {
            Some(surface.as_ref().unwrap())
        } else {
            None
        },
    }).await.unwrap();
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Wolf Engine"), 
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        },
        None
    ).await.unwrap();
    let (surface_config, surface_size) = {
        (None, None)
    };
    GraphicsContext {
        device,
        queue,
        surface,
        surface_config,
        surface_size,
    }
}

pub struct GraphicsContext<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Option<wgpu::Surface<'a>>,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
    surface_size: Option<(u32, u32)>,
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub mod prelude {
    pub use super::GraphicsSettings;
}
