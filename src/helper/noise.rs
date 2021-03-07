use noise::{Fbm, MultiFractal, NoiseFn, Seedable};

pub struct NoiseGenerator {
    noise: Fbm,
}

/// A simple noise generator for creating heights maps
impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        let noise = Fbm::new()
            .set_seed(seed)
            .set_octaves(8)
            .set_frequency(0.03)
            .set_lacunarity(2.0)
            .set_persistence(0.3);

        Self {
            noise
        }
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        self.noise.get([x, y])
    }
}