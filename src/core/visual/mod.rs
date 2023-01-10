mod device;
mod graphics_pipeline;
mod mesh;
mod renderpass;
mod surface;
mod swapchain;

mod process;

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    VulkanLibrary,
};

use winit::event_loop::EventLoop;

use std::sync::Arc;

pub struct VisualEngine {
    vulkan_instance: Arc<Instance>,
    event_loop: EventLoop<()>,
    visual_surface: surface::VisualSurface,
    visual_device: device::VisualDevice,
    visual_swapchain: swapchain::VisualSwapchain,
    visual_render_pass: renderpass::VisualRenderPass,
    visual_graphics_pipeline: graphics_pipeline::VisualGraphicsPipeline,
}

impl VisualEngine {
    pub fn init() {
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
        let visual_render_pass = renderpass::VisualRenderPass::init(
            visual_device.device.clone(),
            &visual_swapchain.swapchain,
        );
        let visual_graphics_pipeline = graphics_pipeline::VisualGraphicsPipeline::init();

        let mut visual_engine: VisualEngine = Self {
            vulkan_instance: instance,
            event_loop: event_loop,
            visual_surface: visual_surface,
            visual_device: visual_device,
            visual_swapchain: visual_swapchain,
            visual_render_pass: visual_render_pass,
            visual_graphics_pipeline: visual_graphics_pipeline,
        };

        visual_engine.process();
    }
}
