use crate::world::{
    map::Map,
    chunk::{
        chunkmanager::ChunkManager,
        pos::*,
    },
    block::blocks::*,
};

use crate::game::{
    player::player::Player,
};

pub struct World {
    pub chunk_manager: ChunkManager,
    pub seed: u32,
    pub player: Player,

    pub map: Map,
}

impl World {
    pub fn new(seed: u32) -> Self {
        let chunk_manager = ChunkManager::new(4);
        let map = Map::new(seed);

        let player = Player::null_player();

        Self {
            chunk_manager,
            seed,
            map,
            player,
        }
    }

    pub fn place_block(&mut self, at: WorldCoord, block: BlockID) {
        match self.chunk_manager.get_chunk_mut_option(at.to_chunk_coord()) {
            None => {println!("Could not place block because chunk was not loaded.");},
            Some(chunk) => {
                chunk.place_block(at.to_chunk_local(), block)
            }
        }
    }

    pub fn remove_block(&mut self, at: WorldCoord) {
        self.place_block(at, Blocks::AIR as BlockID);
    }
}