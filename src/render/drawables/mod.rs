pub mod chunkpos;
pub mod texture_vertex;

use wgpu::{RenderPipeline, RenderPass};

use crate::render::low::renderer::Renderer;

pub trait Drawable {
    fn create_pipeline(renderer: &Renderer) -> RenderPipeline;
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>, renderer: &'a Renderer);
}