use rand::Rng;

use crate::world::block::blocks::{
    Blocks,
    BlockID
};
use crate::world::constants::{CHUNKSIZE, WORLDHEIGHT};
use crate::world::chunk::pos::*;

#[derive(Debug)]
pub struct Chunk {
    /// blocks[x][y][z]
    blocks: [BlockID; CHUNKSIZE * CHUNKSIZE * WORLDHEIGHT],
    pub pos: ChunkPos,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> Self {
        let blocks = [Blocks::AIR as BlockID; CHUNKSIZE * CHUNKSIZE * WORLDHEIGHT];

        Self {
            blocks,
            pos,
        }
    }

    pub fn generate(&mut self, height: [u32; CHUNKSIZE * CHUNKSIZE]) {
        let mut rgn = rand::thread_rng();

        // self.blocks[coord_to_index(0, 0, 0)] = Blocks::GRASS as BlockID;
        // self.blocks[coord_to_index(0, 1, 0)] = Blocks::GRASS as BlockID;

        for x in 0..(CHUNKSIZE) as i16 {
            for z in 0..(CHUNKSIZE) as i16 {

                let grassheight = height[x as usize + z as usize * CHUNKSIZE] as i16;
                let dirtheight = grassheight - rgn.gen_range(1..4);
                let stoneheight = dirtheight;

                let mut y = 0;
                while y < stoneheight {
                    self.blocks[coord_to_index(x, y, z)] = Blocks::STONE as BlockID;
                    y += 1;
                }

                y = dirtheight;
                while y < grassheight {
                    self.blocks[coord_to_index(x, y, z)] = Blocks::DIRT as BlockID;
                    y += 1;
                }

                self.blocks[coord_to_index(x, grassheight, z)] = Blocks::GRASS as BlockID;
            }
        }
    }

    /// Returns the BlockID at a given coordinate inside a chunk
    /// Y represents height, Z depth and X width.
    /// Also makes sure coordinate is in bounds
    pub fn at_coord_bounds(&self, coord: ChunkCoord) -> BlockID {
        if !Chunk::in_bounds(coord) {
            return 0
        }
        else {
            return self.blocks[coord_to_index(coord.x, coord.y, coord.z)]
        }      
    }

    /// This will panic if x, y or z are not in bounds
    pub fn at_coord(&self, coord: ChunkCoord) -> BlockID {
        self.blocks[coord_to_index(coord.x, coord.y, coord.z)]   
    }

    /// Returns true if the given coordinate is in the bounds
    /// of a chunk
    pub fn in_bounds(coord: ChunkCoord) -> bool {
        if 
            coord.x < 0 || coord.x >= CHUNKSIZE as i16 ||
            coord.z < 0 || coord.z >= CHUNKSIZE as i16 ||
            coord.y < 0 || coord.y >= WORLDHEIGHT as i16
        {
            return false
        }

        true
    }

    pub fn place_block(&mut self, pos: ChunkCoord, block: BlockID) {
        self.blocks[coord_to_index(pos.x, pos.y, pos.z)] = block;
    }
}

/// Y represents height, Z depth and X width
pub fn coord_to_index(x: i16, y: i16, z: i16) -> usize {
    ((x + z * CHUNKSIZE as i16) as i32 + y as i32 * (CHUNKSIZE * CHUNKSIZE) as i32) as usize
}

pub fn index_to_coord(index: usize) -> (u32, u32, u32) {
    let x = index % CHUNKSIZE;
    let y = (index / (CHUNKSIZE * CHUNKSIZE)) as u32;
    let z = (index / CHUNKSIZE) % CHUNKSIZE;

    (x as u32, y as u32, z as u32)
}