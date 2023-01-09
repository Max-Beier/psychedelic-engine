use vulkano::{instance::Instance, swapchain::Surface};
use vulkano_win::VkSurfaceBuild;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use std::sync::Arc;

pub struct VisualSurface {
    pub surface: Arc<Surface>,
}

impl VisualSurface {
    pub fn init(instance: Arc<Instance>, event_loop: &EventLoop<()>) -> Self {
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance)
            .unwrap();

        let visual_surface = Self { surface: surface };

        return visual_surface;
    }
}
