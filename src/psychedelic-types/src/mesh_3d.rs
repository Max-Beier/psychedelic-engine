use crate::{
    engine::{MeshBuffer3D, Vertex3D},
    Object3D,
};

pub struct Mesh3D {
    pub extent: Object3D,
    pub mesh_buffer: Option<MeshBuffer3D>,
    pub vertices: Vec<Vertex3D>,
}

impl Mesh3D {
    pub fn new() -> Self {
        return Self {
            extent: Object3D::new(),
            mesh_buffer: None,
            vertices: vec![],
        };
    }
}
