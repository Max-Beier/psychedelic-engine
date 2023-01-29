use std::sync::Arc;

use vulkano::{device::Device, render_pass::RenderPass, swapchain::Swapchain};

pub struct RenderPasses {
    pub normal: Arc<RenderPass>,
}

impl RenderPasses {
    pub fn new(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Self {
        let normal = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.clone().image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();

        return Self { normal: normal };
    }
}
