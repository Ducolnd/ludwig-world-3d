use cgmath::Point2;

use crate::render::low::textures::TextureTile;

pub type BlockID = u16; // A block is a 2 byte unsigned integer

pub const BLOCKS: [Block; 5] = [
    Block {transparent: true, texture: TextureTile {coords: Point2 {x: 14, y: 0}}}, // air
    Block {transparent: false, texture: TextureTile {coords: Point2 {x: 1, y: 0}}}, // Stone
    Block {transparent: false, texture: TextureTile {coords: Point2 {x: 0, y: 0}}}, // Grass
    Block {transparent: false, texture: TextureTile {coords: Point2 {x: 2, y: 0}}}, // Dirt
    Block {transparent: false, texture: TextureTile {coords: Point2 {x: 15, y: 0}}}, // help
];

#[repr(u16)]
pub enum Blocks {
    AIR = 0,
    STONE = 1,
    GRASS = 2,
    DIRT = 3,
}

pub struct Block {
    pub transparent: bool,
    pub texture: TextureTile,
}


pub fn get_block<'a>(id: BlockID) -> &'a Block {
    &BLOCKS[id as usize]
}