use super::State;
use crate::render::{
    low::{
        context::Context,
        renderer::Renderer,
    },
    drawables::{Drawable},
};
use crate::world::{
    chunk::{chunkmanager::ChunkManager, pos::{WorldCoord}},
    world::World,
};

pub struct MainState {
    chm:  ChunkManager,
    world: World,
}

impl State for MainState {
    fn new(renderer: &mut Renderer) -> Self {
        let mut chm = ChunkManager::new(3);
        let world = World::new(69);

        chm.set_camera_location(WorldCoord {x: -1, y: 0, z: 0}, renderer);
        
        Self {
            chm,
            world,
        }
    }

    fn draw(&self) -> Vec<&dyn Drawable> {
        let mut objs = Vec::<&dyn Drawable>::new();

        // Draw all chunks
        for (_, chunk) in &self.chm.chunk_buffers {
            objs.push(chunk);
        }

        objs
    }

    fn update(&mut self, context: &mut Context, encoder: &mut wgpu::CommandEncoder) {
        self.chm.load_queue(&self.world, &mut context.renderer);
        self.chm.update(context, encoder);

        self.chm.set_camera_location(WorldCoord::from_point(context.renderer.camera.view.position), &mut context.renderer);
    }
}