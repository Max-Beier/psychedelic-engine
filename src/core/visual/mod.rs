mod device;
mod surface;
mod swapchain;

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    VulkanLibrary,
};

use winit::event_loop::{self, EventLoop};

use std::sync::Arc;

pub struct VisualEngine {
    vulkan_instance: Arc<Instance>,
    event_loop: EventLoop<()>,
    visual_surface: surface::VisualSurface,
    visual_device: device::VisualDevice,
    visual_swapchain: swapchain::VisualSwapchain,
}

impl VisualEngine {
    pub fn init() -> Self {
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = vulkano_win::required_extensions(&library);

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enumerate_portability: true,
                ..Default::default()
            },
        )
        .unwrap();

        let event_loop = EventLoop::new();

        let visual_surface = surface::VisualSurface::init(instance.clone(), &event_loop);
        let visual_device = device::VisualDevice::init(&instance, &visual_surface.surface);
        let visual_swapchain = swapchain::VisualSwapchain::init(
            visual_device.device.clone(),
            visual_surface.surface.clone(),
        );

        let visual = Self {
            vulkan_instance: instance,
            event_loop: event_loop,
            visual_surface: visual_surface,
            visual_device: visual_device,
            visual_swapchain: visual_swapchain,
        };

        return visual;
    }
}
