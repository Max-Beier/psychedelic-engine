use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    library, VulkanLibrary,
};

use std::sync::Arc;

pub struct VisualEngine {
    vulkan_instance: Arc<Instance>,
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

        let visual = Self {
            vulkan_instance: instance,
        };

        return visual;
    }
}
