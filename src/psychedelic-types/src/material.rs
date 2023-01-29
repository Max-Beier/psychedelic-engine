use vulkano::shader::ShaderModule;

pub struct Material {
    pub vertex_shader: Option<ShaderModule>,
    pub fragment_shader: Option<ShaderModule>,

    // MATERIAL PROPERTIES
    pub albedo: [f32; 3],
}

impl Material {
    pub fn new() -> Self {
        return Self {
            vertex_shader: None,
            fragment_shader: None,

            albedo: [1.0, 1.0, 1.0],
        };
    }
}
