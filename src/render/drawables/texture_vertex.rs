use crate::render::low::{
    renderer::Renderer, 
    buffer::DynamicBuffer, 
    vertex,
};
use crate::render::{
    shapes::shape::Shape,
    vertexarray::VertexArray,
};

use super::Drawable;

pub struct TextureVertex {
    vertex_buffer: DynamicBuffer<vertex::Vertex>,
    index_buffer: DynamicBuffer<u32>,
}

impl TextureVertex {
    #[allow(dead_code)]    
    pub fn new(device: &wgpu::Device) -> Self {
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
        }
    }

    #[allow(dead_code)]
    pub fn from_vertex_array<T: Shape>(&mut self, vertex_array: &VertexArray<T>, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.vertex_buffer.insert_back(
            device, 
            encoder,
            &vertex_array.to_vertices(),
        );

        self.index_buffer.insert_back(
            device, 
            encoder,
            &vertex_array.to_indices(),
        );
    }
}

impl Drawable for TextureVertex {
    fn create_pipeline(renderer: &Renderer) -> wgpu::RenderPipeline {
        renderer.default_pipeline(
            wgpu::include_spirv!("../low/shaders/texture_vertex.vert.spv"),
            wgpu::include_spirv!("../low/shaders/texture_vertex.frag.spv"), 
            &[
                &renderer.camera.uniform.uniform_bind_group_layout, // set = 0
                &renderer.textures.texture_bind_group_layout, // set = 1
            ],
        )
    }

    fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, renderer: &'a Renderer) {
        pass.set_pipeline(renderer.get_pipeline::<Self>());
        pass.set_bind_group(renderer.camera.uniform.index, &renderer.camera.uniform.uniform_bind_group, &[]); // Camera
        pass.set_bind_group(1, renderer.textures.get_bind_group(), &[]); // Texture

        pass.set_vertex_buffer(0, self.vertex_buffer.get_buffer().slice(..));
        pass.set_index_buffer(self.index_buffer.get_buffer().slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_buffer.len as u32, 0, 0..1);
    }
}