mod initialisation;
mod process;
mod utils;

use std::sync::Arc;
use utils::Vertex;

use vulkano::{
    buffer::CpuAccessibleBuffer,
    device::{Device, Queue},
    pipeline::{graphics::viewport::Viewport, GraphicsPipeline},
    render_pass::{Framebuffer, RenderPass},
    swapchain::{Surface, Swapchain},
};
use winit::event_loop::EventLoop;

pub struct VisualEngine {
    event_loop: EventLoop<()>,
    surface: Arc<Surface>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    render_pass: Arc<RenderPass>,
    graphics_pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
}
