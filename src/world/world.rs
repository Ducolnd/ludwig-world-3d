use crate::world::{
    map::Map,
};

use crate::game::{
    player::player::Player,
};

pub struct World {
    // pub chunk_manager: ChunkManager,
    pub seed: u32,
    pub player: Player,

    pub map: Map,
}

impl World {
    pub fn new(seed: u32) -> Self {
        // let chunk_manager = ChunkManager::new(RENDER_DISTANCE as u32);
        let map = Map::new(seed);

        let player = Player::null_player();

        Self {
            // chunk_manager,
            seed,
            map,
            player,
        }
    }
}