use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

struct ViewportDesc {
    window: Arc<Window>,
    background: wgpu::Color,
    surface: wgpu::Surface<'static>,
}

impl ViewportDesc {
    fn new(window: Arc<Window>, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).unwrap();
        Self {
            window,
            background,
            surface,
        }
    }

    fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Viewport {
        let size = self.window.inner_size();
        let config = self
            .surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();
        self.surface.configure(device, &config);
        Viewport { desc: self, config }
    }
}

struct Viewport {
    desc: ViewportDesc,
    config: wgpu::SurfaceConfiguration,
}

impl Viewport {
    fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }
    fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
}

pub struct WgpuContext {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct App {
    viewport: Option<Viewport>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    window: Option<Arc<Window>>,
    instance: Arc<wgpu::Instance>,
    adpter: Option<wgpu::Adapter>,
    runtime: tokio::runtime::Runtime,
}

impl App {
    fn new() -> Self {
        let instance = Arc::new(wgpu::Instance::new(wgpu::InstanceDescriptor::default()));
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        Self {
            viewport: None,
            device: None,
            queue: None,
            window: None,
            adpter: None,
            instance,
            runtime,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(Window::default_attributes()) {
            self.window = Some(window.into());
        };
        let instance = self.instance.clone();
        let window = self.window.clone().unwrap();
        let viewport_desc = ViewportDesc::new(window, wgpu::Color::BLUE, &self.instance);
        let (viewport, device, adapter, quene) = self.runtime.block_on(async move {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    compatible_surface: Some(&viewport_desc.surface),
                    ..Default::default()
                })
                .await
                .unwrap();
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::downlevel_defaults(),
                    },
                    None,
                )
                .await
                .unwrap();
            let viewport = viewport_desc.build(&adapter, &device);
            (viewport, device, adapter, queue)
        });
        self.viewport = Some(viewport);
        self.device = Some(device);
        self.queue = Some(quene);
        self.adpter = Some(adapter);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let mut device = self.device.as_mut().unwrap();
        let mut queue = self.queue.as_mut().unwrap();
        let mut viewport = self.viewport.as_mut().unwrap();
        let mut adapter = self.adpter.as_mut().unwrap();
        let max_buffer_size = adapter.limits().max_buffer_size;
        match event {
            WindowEvent::Resized(new_size) => {
                if max_buffer_size < (new_size.width * new_size.height) as u64 {
                    return;
                }
                // Recreate the swap chain with the new size
                viewport.resize(device, new_size);
                // On macos the window needs to be redrawn manually after resizing
                viewport.desc.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let frame = viewport.get_current_texture();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(viewport.desc.background),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
