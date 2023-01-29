use std::sync::Arc;

use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator},
};

use super::Vertex3D;

pub struct MeshBuffer3D {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex3D]>>,
}

impl MeshBuffer3D {
    pub fn new(
        memory_allocator: &GenericMemoryAllocator<Arc<FreeListAllocator>>,
        vertices: Vec<Vertex3D>,
    ) -> Self {
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

        let uniform_buffer = CpuAccessibleBuffer::from_iter(
            memory_allocator,
            BufferUsage {
                uniform_buffer: true,
                ..Default::default()
            },
            false,
            vec![0.0],
        );

        return Self {
            vertex_buffer: vertex_buffer,
        };
    }
}
