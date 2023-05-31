use anyhow::{Ok, Result};
use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[derive(Debug)]
enum ErrorType {
    AdapterError(String)
}
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ErrorType {}

async fn run(event_loop: EventLoop<()>, window: Window) -> Result<()> {
    let size = window.inner_size();
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&window) }?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await.ok_or(ErrorType::AdapterError("bruh".into()))?;

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::downlevel_defaults(),
    }, None).await?;

    Ok(())
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    block_on(run(event_loop, window))?;

    Ok(())
}
