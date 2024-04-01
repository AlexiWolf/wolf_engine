pub use wgpu;

use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle + Send + Sync {}

impl<T> WindowHandle for T where T: HasDisplayHandle + HasWindowHandle + Send + Sync {}

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
        window: Option<(&dyn WindowHandle, (u32, u32))>,
    ) -> Result<GraphicsContext, &'static str> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = match window {
            Some((window, _)) => instance.create_surface(window).ok(),
            None => None,
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: surface.as_ref(),
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
        let surface_config = match window {
            Some((_, size)) => {
                let surface = surface.as_ref().unwrap();
                let surface_capabilities = surface.get_capabilities(&adapter);
                let surface_format = surface_capabilities
                    .formats
                    .iter()
                    .copied()
                    .filter(|f| f.is_srgb())
                    .next()
                    .unwrap_or(surface_capabilities.formats[0]);
                let surface_config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: surface_format,
                    width: size.0,
                    height: size.1,
                    present_mode: surface_capabilities.present_modes[0],
                    desired_maximum_frame_latency: 0,
                    alpha_mode: surface_capabilities.alpha_modes[0],
                    view_formats: vec![],
                };
                surface.configure(&device, &surface_config);
                Some(surface_config)
            }
            None => None,
        };
        Ok(GraphicsContext {
            device,
            queue,
            surface,
            surface_config,
        })
    }
}

pub struct GraphicsContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Option<wgpu::Surface<'window>>,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
}

impl GraphicsContext<'_> {
    pub fn resize(&mut self, _width: u32, _height: u32) {}

    pub fn new_frame(&mut self) -> Option<Frame> {
        if let Some(surface) = self.surface.as_ref() {
            let output = surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            Some(Frame {
                output,
                view,
                encoder,
            })
        } else {
            None
        }
    }

    pub fn clear(&mut self, frame: &mut Frame, color: wgpu::Color) {
        let _render_pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
    }

    pub fn present(&mut self, frame: Frame) {
        self.queue.submit(std::iter::once(frame.encoder.finish()));
        frame.output.present();
    }
}

pub struct Frame {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

#[cfg(test)]
mod graphics_init_tests {
    #[pollster::test]
    async fn should_use_builder_pattern() {
        let _graphics = crate::init().build(None).await.unwrap();
    }
}
