use cgmath::Vector3;

use crate::{
    engine::{MeshBuffer3D, Vertex3D},
    Material, Object3D,
};

pub struct Mesh3D {
    pub extent: Object3D,
    pub mesh_buffer: Option<MeshBuffer3D>,
    pub vertices: Vec<Vertex3D>,
    pub size: Vector3<f32>,
    pub material: Material,
}

impl Mesh3D {
    pub fn new() -> Self {
        return Self {
            extent: Object3D::new(),
            mesh_buffer: None,
            vertices: vec![],
            size: Vector3::new(0.0, 0.0, 0.0),
            material: Material::new(),
        };
    }
}
