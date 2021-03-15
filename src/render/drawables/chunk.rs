use crate::render::{
    low::{
        buffer::DynamicBuffer,
        renderer::Renderer,
        vertex,
    },
    meshing::chunkmeshing::ChunkMesh,
};
use crate::world::chunk::pos::ChunkPos;
use super::Drawable;

pub struct ChunkDrawable {
    vertex_buffer: DynamicBuffer<vertex::Vertex>,
    index_buffer: DynamicBuffer<u32>,
    pos: ChunkPos,
}

impl ChunkDrawable {
    pub fn new(device: &wgpu::Device, pos: ChunkPos) -> Self {
        let vertex_buffer = DynamicBuffer::new(
            8000,
            device,
            wgpu::BufferUsage::VERTEX,
        );

        let index_buffer = DynamicBuffer::new(
            8000,
            device,
            wgpu::BufferUsage::INDEX,
        );

        Self {
            vertex_buffer,
            index_buffer,
            pos,
        }
    }

    pub fn from_chunk_mesh(&mut self, mesh: &ChunkMesh, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.vertex_buffer.insert_back(
            device, 
            encoder,
            &mesh.to_vertex_array().to_vertices(),
        );

        self.index_buffer.insert_back(
            device, 
            encoder,
            &mesh.to_vertex_array().to_indices(),
        );
    }
}

impl Drawable for ChunkDrawable {
    fn create_pipeline(renderer: &Renderer) -> wgpu::RenderPipeline {
        renderer.default_pipeline(
            wgpu::include_spirv!("../low/shaders/chunk.vert.spv"),
            wgpu::include_spirv!("../low/shaders/chunk.frag.spv"), 
            &[
                &renderer.camera.uniform.uniform_bind_group_layout, // set = 0
                &renderer.textures.texture_bind_group_layout, // set = 1
                &renderer.chunkpos_uniform.uniform_bind_group_layout, // set = 2
            ],
        )
    }

    fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, renderer: &'a Renderer) {
        pass.set_pipeline(renderer.get_pipeline::<Self>());
        pass.set_bind_group(renderer.camera.uniform.index, &renderer.camera.uniform.uniform_bind_group, &[]); // Camera
        pass.set_bind_group(1, renderer.textures.get_bind_group(), &[]); // Texture
        
        // Set correct chunkpos uniform
        let a = renderer.chunkpos_uniform.offset.get(&self.pos).unwrap() * wgpu::BIND_BUFFER_ALIGNMENT as u32;
        pass.set_bind_group(renderer.chunkpos_uniform.index, &renderer.chunkpos_uniform.uniform_bind_group, &[a]);

        // Draw
        pass.set_vertex_buffer(0, self.vertex_buffer.get_buffer().slice(..));
        pass.set_index_buffer(self.index_buffer.get_buffer().slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_buffer.len as u32, 0, 0..1);
    }
}