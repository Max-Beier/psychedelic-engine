mod initialisation;
mod process;
mod render_passes;
mod utils;

use psychedelic_types::{engine::MeshBuffer3D, Mesh3D};
use std::sync::Arc;

use vulkano::{
    device::{Device, Queue},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator},
    pipeline::{graphics::viewport::Viewport, GraphicsPipeline},
    render_pass::Framebuffer,
    swapchain::{Surface, Swapchain},
};
use winit::event_loop::EventLoop;

pub struct VisualEngine {
    event_loop: EventLoop<()>,
    loaded_meshes: Vec<Mesh3D>,
    surface: Arc<Surface>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    memory_allocator: GenericMemoryAllocator<Arc<FreeListAllocator>>,
    render_passes: render_passes::RenderPasses,
    graphics_pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl VisualEngine {
    pub fn load_mesh(&mut self, mut mesh: Mesh3D) {
        if mesh.mesh_buffer.is_none() {
            mesh.mesh_buffer = Some(MeshBuffer3D::new(
                &self.memory_allocator,
                mesh.vertices.clone(),
            ));
        }
        self.loaded_meshes.push(mesh);
    }
}
