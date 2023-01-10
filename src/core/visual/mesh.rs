use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::StandardMemoryAllocator,
};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 2],
}

pub struct VisualMesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl VisualMesh {
    fn init(memory_allocator: &StandardMemoryAllocator) -> Self {
        let vertices = [
            Vertex {
                position: [-0.5, -0.25],
            },
            Vertex {
                position: [0.0, 0.5],
            },
            Vertex {
                position: [0.25, -0.1],
            },
        ];

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            memory_allocator,
            BufferUsage {
                vertex_buffer: true,
                ..Default::default()
            },
            false,
            vertices,
        )
        .unwrap();

        let visual_mesh = Self {
            vertex_buffer: vertex_buffer,
        };

        return visual_mesh;
    }
}
