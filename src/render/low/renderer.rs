use wgpu;
use std::collections::HashMap;

use crate::render::low::{
    init,
    buffer,
    vertex,
    textures::TextureManager,
    uniforms::Uniform,
    uniforms::{ChunkPositionUniform, CameraUniform, MultiUniform},
    buffer::DynamicBuffer,
};
use crate::world::chunk::pos::ChunkPos;

/// The Renderer holds buffers and pipelines.
/// It renders everything.
pub struct Renderer {  
    // Render
    pub vertex_buffer: HashMap<ChunkPos, buffer::DynamicBuffer<vertex::Vertex>>,
    pub index_buffer: HashMap<ChunkPos, buffer::DynamicBuffer<u32>>,

    pub depth_view: wgpu::TextureView,

    // .. more pipelines more buffers ..
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        camera_bind_group: &wgpu::BindGroupLayout,
        chunkpos_bind_group: &wgpu::BindGroupLayout,
        texture_manager: &TextureManager,
    ) -> Self {

        // Main rendering pipeline
        let render_pipeline = init::default_render_pipeline(
            device, 
            sc_desc,
            &[
                camera_bind_group,  // Camera set=0
                chunkpos_bind_group, // Chunkpos set=1
                &texture_manager.texture_bind_group_layout,  // Texture set=2
            ],
        );

        // Init empty buffers
        let vertex_buffer = HashMap::new();
        let index_buffer = HashMap::new();

        let (_, depth_view, _) = init::default_depth_texture(device, sc_desc);

        // Return
        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,

            depth_view,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        swap_chain: &mut wgpu::SwapChain,
        queue: &wgpu::Queue,
        camera_uniform: &Uniform<CameraUniform>,
        chunkpos_uniform: &mut MultiUniform<ChunkPos, ChunkPositionUniform>,
        texture_manager: &TextureManager,
    ) -> Result<(), wgpu::SwapChainError> {

        let frame = swap_chain.get_current_frame()?.output;
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass descriptor in renderer"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(camera_uniform.index, &camera_uniform.uniform_bind_group, &[]); // Camera
            render_pass.set_bind_group(2, texture_manager.get_bind_group(), &[]); // Texture

            for (pos, buffer) in &self.vertex_buffer {
                let a = *chunkpos_uniform.offset.get(&pos).unwrap() * wgpu::BIND_BUFFER_ALIGNMENT as u32;
                render_pass.set_bind_group(chunkpos_uniform.index, &chunkpos_uniform.uniform_bind_group, &[a]); // Chunk Positions uniform changes with every chunk

                render_pass.set_vertex_buffer(0, buffer.get_buffer().slice(..));
                render_pass.set_index_buffer(self.index_buffer.get(&pos).unwrap().get_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..self.index_buffer.get(&pos).unwrap().len as u32, 0, 0..1);
            }
        }
    
        queue.submit(vec![encoder.finish()]);
    
        Ok(())
    }
}