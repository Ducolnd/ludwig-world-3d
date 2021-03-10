use wgpu::{RenderPipeline, RenderPass};

use crate::render::low::master::Master;

pub trait Drawable {
    fn create_pipeline(master: &mut Master) -> RenderPipeline;
    fn draw(pass: &mut RenderPass);
}