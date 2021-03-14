pub mod chunk;
pub mod texture_vertex;

use wgpu::{RenderPipeline, RenderPass};

use crate::render::low::renderer::Renderer;

pub trait Drawable {
    /// Initialize all related things such as a uniform buffer specific for the Drawable
    /// and the renderpipeline.
    fn create_pipeline(renderer: &Renderer) -> RenderPipeline;
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>, renderer: &'a Renderer);
}