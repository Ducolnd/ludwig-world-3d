use std::collections::HashMap;
use std::time::Instant;

use crate::render::low::master::Master;
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::world::chunk::chunk::{Chunk};
use crate::world::chunk::pos::*;
use crate::world::chunk::chunkmanager::ChunkManager;
use crate::world::constants::RENDER_DISTANCE;

pub struct World {
    pub chunk_manager: ChunkManager,
    pub seed: u32,
}

impl World {
    pub fn new(seed: u32) -> Self {
        let chunk_manager = ChunkManager::init(RENDER_DISTANCE as u32);

        Self {
            chunk_manager,
            seed,
        }
    }
}