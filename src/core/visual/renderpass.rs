use std::sync::Arc;
use vulkano::{device::Device, render_pass::RenderPass, swapchain::Swapchain};

pub struct VisualRenderPass {
    pub render_passes: Vec<Arc<RenderPass>>,
}

impl VisualRenderPass {
    pub fn init(device: Arc<Device>, swapchain: &Arc<Swapchain>) -> Self {
        let render_pass = vulkano::single_pass_renderpass!(device, attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();

        let visual_render_pass = Self {
            render_passes: vec![render_pass],
        };

        return visual_render_pass;
    }
}
