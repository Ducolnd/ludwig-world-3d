use std::collections::HashMap;

use super::State;
use crate::render::{
    low::{
        renderer::Renderer,
        context::Context,
    },
    drawables::{Drawable, chunk::ChunkDrawable},
};
use crate::world::{
    chunk::{chunkmanager::ChunkManager, pos::ChunkPos},
};

pub struct MainState {
    chm:  ChunkManager,
    chunks: HashMap<ChunkPos, ChunkDrawable>,

    loaded: bool, 
}

impl State for MainState {
    fn new(renderer: &mut Renderer) -> Self {
        let mut chm = ChunkManager::new(2);

        let pos = ChunkPos::new(0, 0, 0);
        chm.load_chunk(pos, [4; 16*16], renderer);
        chm.load_chunk(ChunkPos::new(1, 0, 0), [3; 16*16], renderer);

        let chunks = HashMap::new();
        
        Self {
            chm,
            chunks,
            loaded: false,
        }
    }

    fn draw(&self, renderer: &Renderer) -> Vec<&dyn Drawable> {
        let mut objs = Vec::<&dyn Drawable>::new();

        for (_, chunk) in &self.chunks {
            objs.push(chunk);
        }

        objs
    }

    fn update(&mut self, context: &Context, encoder: &mut wgpu::CommandEncoder) {
        if !self.loaded {
            for (pos, chunk) in &self.chm.chunks_meshes {
                let mut c = ChunkDrawable::new(&context.renderer.device, *pos);
                c.from_chunk_mesh(&chunk, &context.renderer.device, encoder);

                self.chunks.insert(*pos, c);
            }
        }
    }
}