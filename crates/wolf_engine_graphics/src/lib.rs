use wgpu::rwh::HasWindowHandle;

pub fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {
        _settings: GraphicsSettings::default(),
    }
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub struct GraphicsContextBuilder {
    _settings: GraphicsSettings,
}

impl GraphicsContextBuilder {
    pub async fn build(
        self,
        _window: Option<&dyn HasWindowHandle>,
    ) -> Result<GraphicsContext, &'static str> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        Ok(GraphicsContext {})
    }
}

pub struct GraphicsContext {}

impl GraphicsContext {
    pub fn resize(&mut self, _width: u32, _height: u32) {}
}

#[cfg(test)]
mod graphics_init_tests {
    #[pollster::test]
    async fn should_use_builder_pattern() {
        let _graphics = crate::init().build(None).await.unwrap();
    }
}
