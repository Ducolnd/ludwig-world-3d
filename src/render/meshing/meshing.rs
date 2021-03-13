use crate::render::vertexarray::VertexArray;
use crate::render::shapes::shapes::Quad;
use crate::world::block::blocks::{BlockID, get_block, Sides};
use crate::render::{
    low::{
        renderer::Renderer,
    },
};

// Used for creating the corresponding faces. These represent coordinates of the 4 vertices in the correct order
pub const FACES: [Face; 6] = [
    Face {interval: [[0, 1, 0], [1, 1, 0], [1, 0, 0], [0, 0, 0]]},
    Face {interval: [[1, 1, 0], [1, 1, 1], [1, 0, 1], [1, 0, 0]]},
    Face {interval: [[1, 1, 1], [0, 1, 1], [0, 0, 1], [1, 0, 1]]},
    Face {interval: [[0, 1, 1], [0, 1, 0], [0, 0, 0], [0, 0, 1]]},
    Face {interval: [[0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0]]},
    Face {interval: [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]]},
];

/// A mesh holds quads and renders these quads
pub struct Mesh {
    pub quads: VertexArray<Quad>,
}

impl Mesh {
    pub fn new() -> Self {
        let quads = VertexArray::new();

        Self {
            quads,
        }
    }

    pub fn add_face(&mut self, face: MeshFace) {
        let interval = &FACES[face.face as usize];
        self.quads.push(Quad {
            coords: [
                [(face.coordinate[0] + interval.interval[0][0]) as f32, (face.coordinate[1] + interval.interval[0][1]) as f32, (face.coordinate[2] + interval.interval[0][2]) as f32],
                [(face.coordinate[0] + interval.interval[1][0]) as f32, (face.coordinate[1] + interval.interval[1][1]) as f32, (face.coordinate[2] + interval.interval[1][2]) as f32],
                [(face.coordinate[0] + interval.interval[2][0]) as f32, (face.coordinate[1] + interval.interval[2][1]) as f32, (face.coordinate[2] + interval.interval[2][2]) as f32],
                [(face.coordinate[0] + interval.interval[3][0]) as f32, (face.coordinate[1] + interval.interval[3][1]) as f32, (face.coordinate[2] + interval.interval[3][2]) as f32],
            ],
            text_coords: get_block(face.blocktype).texture.sides[face.face as usize].to_usable(),
        })
    }
}

pub struct Face {
    interval: [[u32; 3]; 4],
}

pub struct MeshFace {
    pub coordinate: [u32; 3],
    pub face: Sides, // Which face of block
    pub blocktype: BlockID,
}