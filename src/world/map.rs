use crate::world::constants::{*};
use crate::world::chunk::pos::ChunkPos;
use crate::helper::noise::NoiseGenerator;

/// The Map takes care of generating the world
/// and it contains and hands out heightmaps
pub struct Map {
    noise_generator: NoiseGenerator,
}

impl Map {
    pub fn new(seed: u32) -> Self {

        let noise_generator = NoiseGenerator::new(seed);
        
        Self {
            noise_generator,
        }
    }

    pub fn create_heightmap(&self, chunk: &ChunkPos) -> [u32; CHUNKSIZE * CHUNKSIZE] {
        let mut heights = [0; CHUNKSIZE * CHUNKSIZE];

        for x in 0..CHUNKSIZE {
            for z in 0..CHUNKSIZE {
                heights[x as usize + z as usize * CHUNKSIZE] = ((self.noise_generator.get(x as f64 + chunk.x as f64 * CHUNKSIZE as f64, z as f64 + chunk.z as f64 * CHUNKSIZE as f64) * 20.0) as i32 + 20) as u32;
            }
        }

        // println!("{:?}", heights);

        heights
    }
}