use crate::world::constants::{WORLD_WIDTH, WORLD_HEIGHT};
use crate::world::chunk::pos::ChunkPos;
use crate::helper::noise::NoiseGenerator;

/// The Map takes care of generating the world
/// and it contains and hands out heightmaps
pub struct Map {
    noise_generator: NoiseGenerator,

    height_map: [[f64; WORLD_WIDTH as usize]; WORLD_HEIGHT as usize],
}

impl Map {
    pub fn new(seed: u32) -> Self {

        let noise_generator = NoiseGenerator::new(seed);
        let height_map = [[0.0; WORLD_WIDTH as usize]; WORLD_HEIGHT as usize];

        
        Self {
            noise_generator,

            height_map,
        }
    }

    pub fn create_heightmap(&mut self) {
        for x in 0..WORLD_WIDTH {
            for z in 0..WORLD_HEIGHT {
                self.height_map[x as usize][z as usize] = self.noise_generator.get(x as f64, z as f64);
            }
        }
    }

    pub fn get_height(&self, pos: ChunkPos) -> f64 {
        self.height_map[pos.x as usize][pos.z as usize]
    }
}