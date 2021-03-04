use std::ops::{Add, Sub};

use crate::world::constants::*;
use crate::render::low::uniforms::ChunkPositionUniform;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Chunk coordinate in world so (0, 0) is from 
/// x 0 to 16 and y 0 to 16
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,y,z,
        }
    }

    pub fn to_raw(&self) -> ChunkPositionUniform {
        ChunkPositionUniform{ location: [
            (self.x * CHUNKSIZE as i32) as f32, 
            (self.y * 1 as i32) as f32, 
            (self.z * CHUNKSIZE as i32) as f32, ] 
        }
    }
}

impl Add for ChunkPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for ChunkPos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Coordinate inside a chunk ranging from 0 to CHUNKSIZE
pub struct ChunkCoord {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

/// Global world position
/// this is not used for entities
pub struct WorldCoord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl WorldCoord {
    pub fn to_chunk_local(&self) -> ChunkCoord {
        ChunkCoord {
            x: (CHUNKSIZE as u16 + (self.x % CHUNKSIZE as i64) as u16) % CHUNKSIZE as u16,
            y: (WORLDHEIGHT as u16 + (self.y % WORLDHEIGHT as i64) as u16) % WORLDHEIGHT as u16,
            z: (CHUNKSIZE as u16+ (self.z % CHUNKSIZE as i64) as u16) % CHUNKSIZE as u16,
        }
    }
}
