use crate::render::{
    low::{
        context::Context,
        renderer::Renderer,
    },
    drawables::Drawable,
};

pub trait State {
    fn new(renderer: &mut Renderer) -> Self;
    /// Update all state
    fn update(&mut self, context: &mut Context, encoder: &mut wgpu::CommandEncoder);
    /// Draw all state such as chunks, ui, players, mobs.
    fn draw(&self) -> Vec<&dyn Drawable>;
}

pub mod mainstate;