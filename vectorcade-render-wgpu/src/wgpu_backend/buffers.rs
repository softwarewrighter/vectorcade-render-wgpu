//! GPU buffer management with pooling and dynamic resizing.

use crate::tessellate::Vertex;

/// Manages vertex and index buffers with capacity tracking.
pub struct BufferPool {
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    vertex_capacity: usize,
    index_capacity: usize,
}

impl BufferPool {
    /// Initial buffer capacity (number of vertices/indices).
    const INITIAL_VERTICES: usize = 4096;
    const INITIAL_INDICES: usize = 8192;

    /// Create a new buffer pool with default capacity.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            vertex: create_vertex_buffer(device, Self::INITIAL_VERTICES),
            index: create_index_buffer(device, Self::INITIAL_INDICES),
            vertex_capacity: Self::INITIAL_VERTICES,
            index_capacity: Self::INITIAL_INDICES,
        }
    }

    /// Update buffers with new geometry, resizing if needed.
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Vertex],
        indices: &[u32],
    ) {
        if vertices.len() > self.vertex_capacity {
            self.vertex_capacity = next_capacity(vertices.len());
            self.vertex = create_vertex_buffer(device, self.vertex_capacity);
        }
        if indices.len() > self.index_capacity {
            self.index_capacity = next_capacity(indices.len());
            self.index = create_index_buffer(device, self.index_capacity);
        }
        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex, 0, bytemuck::cast_slice(vertices));
        }
        if !indices.is_empty() {
            queue.write_buffer(&self.index, 0, bytemuck::cast_slice(indices));
        }
    }
}

fn create_vertex_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: (capacity * std::mem::size_of::<Vertex>()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_index_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Index Buffer"),
        size: (capacity * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn next_capacity(required: usize) -> usize {
    required.next_power_of_two()
}
