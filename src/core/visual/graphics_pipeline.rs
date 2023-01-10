use vulkano::pipeline::GraphicsPipeline;

pub struct VisualGraphicsPipeline {}

impl VisualGraphicsPipeline {
    pub fn init() -> Self {
        let pipeline = GraphicsPipeline::start();

        // TODO: merging subpasses and shaders into the pipeline

        let visual_graphics_pipeline = Self {};

        return visual_graphics_pipeline;
    }
}
