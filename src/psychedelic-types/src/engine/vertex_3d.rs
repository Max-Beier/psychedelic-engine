use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex3D {
    pub position: [f32; 3],
}

vulkano::impl_vertex!(Vertex3D, position);
