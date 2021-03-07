use cgmath::Point2;

use crate::render::low::textures::TextureTile;

pub type BlockID = u16; // A block is a 2 byte unsigned integer

pub const BLOCKS: [Block; 5] = [
    Block {transparent: true, texture: TextureSides::single_layout(TextureTile::new(14, 0))}, // air
    Block {transparent: false, texture: TextureSides::single_layout(TextureTile::new(1, 0))}, // Stone
    Block {transparent: false, texture: TextureSides::normal_layout(TextureTile::new(0, 0), TextureTile::new(2, 0), TextureTile::new(3, 0))}, // Grass
    Block {transparent: false, texture: TextureSides::single_layout(TextureTile::new(2, 0))}, // Dirt
    Block {transparent: false, texture: TextureSides::single_layout(TextureTile::new(15, 0))}, // help
];

#[derive(Debug, Copy, Clone)]
/// This array must correspond to the FACES array in meshing.rs
pub enum Sides {
    FRONT = 0,
    LEFT = 1, 
    BACK = 2,
    RIGHT = 3,
    TOP = 4,
    BOTTOM = 5,
}

#[repr(u16)]
pub enum Blocks {
    AIR = 0,
    STONE = 1,
    GRASS = 2,
    DIRT = 3,
}

pub struct TextureSides {
    pub sides: [TextureTile; 6],
}

impl TextureSides {
    const fn single_layout(text: TextureTile) -> Self {
        Self {
            sides: [text; 6],
        }
    }

    const fn normal_layout(top: TextureTile, bottom: TextureTile, side: TextureTile) -> Self {
        Self {
            sides: [side, side, side, side, top, bottom],
        }
    }
}

pub struct Block {
    pub transparent: bool,
    pub texture: TextureSides,
}

pub fn get_block<'a>(id: BlockID) -> &'a Block {
    &BLOCKS[id as usize]
}