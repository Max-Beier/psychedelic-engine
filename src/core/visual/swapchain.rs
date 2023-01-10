use vulkano::{
    device::Device,
    image::ImageUsage,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};
use winit::window::Window;

use std::sync::Arc;

pub struct VisualSwapchain {
    pub swapchain: Arc<Swapchain>,
}

impl VisualSwapchain {
    pub fn init(device: Arc<Device>, surface: Arc<Surface>) -> Self {
        let (mut swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            Swapchain::new(
                device,
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage {
                        color_attachment: true,
                        ..Default::default()
                    },
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        let visual_swapchain = Self {
            swapchain: swapchain,
        };
        return visual_swapchain;
    }

    pub fn recreate(&self) {}
}
