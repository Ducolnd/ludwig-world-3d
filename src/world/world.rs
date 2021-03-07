use std::collections::HashMap;
use std::time::Instant;

use crate::render::low::master::Master;
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::world::chunk::pos::ChunkPos;

use crate::world::{
    chunk::{
        chunk::Chunk,
        pos,
        chunkmanager::ChunkManager,
    },
    constants::RENDER_DISTANCE,
    map::Map,
};

pub struct World {
    pub chunk_manager: ChunkManager,
    pub seed: u32,

    map: Map,
}

impl World {
    pub fn new(seed: u32) -> Self {
        let chunk_manager = ChunkManager::init(RENDER_DISTANCE as u32);
        let mut map = Map::new(seed);

        Self {
            chunk_manager,
            seed,
            map,
        }
    }

    pub fn place_player(&mut self, pos: ChunkPos, master: &mut Master) {
        self.load_chunk(pos, master);
        self.load_chunk(ChunkPos {x: pos.x + 1, ..pos}, master);
        self.load_chunk(ChunkPos {x: pos.x + 2, ..pos}, master);
        self.load_chunk(ChunkPos {x: pos.x + 3, ..pos}, master);
        self.load_chunk(ChunkPos {z: pos.z + 1, ..pos}, master);
        self.load_chunk(ChunkPos {z: pos.z - 1, ..pos}, master);
    }

    fn load_chunk(&mut self, pos: ChunkPos, master: &mut Master) {
        self.chunk_manager.load_chunk(pos, master, self.map.create_heightmap(pos));
    }
}