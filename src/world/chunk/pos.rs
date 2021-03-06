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

#[derive(Debug, Clone, Copy)]
/// Coordinate inside a chunk ranging from 0 to CHUNKSIZE and WORLDHEIGHT
pub struct ChunkCoord {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Clone, Copy)]
/// Global world position
/// this is not used for entities
pub struct WorldCoord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl WorldCoord {
    pub fn from_chunk_pos(chunkpos: ChunkPos, chunkcoord: ChunkCoord) -> Self {
        Self {
            x: (chunkpos.x * (CHUNKSIZE as i32 ) + chunkcoord.x as i32) as i64,
            y: (chunkpos.y * (1 as i32 ) + chunkcoord.y as i32) as i64,
            z: (chunkpos.z * (CHUNKSIZE as i32 ) + chunkcoord.z as i32) as i64,
        }
    }

    pub fn to_chunk_local(&self) -> ChunkCoord {
        ChunkCoord {
            x: (CHUNKSIZE as i16 + (self.x % CHUNKSIZE as i64) as i16) % CHUNKSIZE as i16,
            y: (WORLDHEIGHT as i16 + (self.y % WORLDHEIGHT as i64) as i16) % WORLDHEIGHT as i16,
            z: (CHUNKSIZE as i16+ (self.z % CHUNKSIZE as i64) as i16) % CHUNKSIZE as i16,
        }
    }

    pub fn to_chunk_coord(&self) -> ChunkPos {
        ChunkPos {
            x: (self.x as f64 / CHUNKSIZE as f64).floor() as i32,
            y: 0,
            z: (self.z as f64 / CHUNKSIZE as f64).floor() as i32,
        }
    }
}
