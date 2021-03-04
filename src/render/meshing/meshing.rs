use crate::render::vertexarray::VertexArray;
use crate::render::shapes::shapes::Quad;
use crate::world::block::blocks::{BlockID, get_block};
use crate::render::low::textures::TextureTile;

// Used for creating the corresponding faces. These represent coordinates of the 4 vertices
pub const FRONT_FACE: Face = Face {interval: [[0, 1, 0], [1, 1, 0], [1, 0, 0], [0, 0, 0]]}; #[allow(dead_code)]
pub const LEFT_FACE: Face = Face {interval: [[1, 1, 0], [1, 1, 1], [1, 0, 1], [1, 0, 0]]}; #[allow(dead_code)]
pub const BACK_FACE: Face = Face {interval: [[1, 1, 1], [0, 1, 1], [0, 0, 1], [1, 0, 1]]}; #[allow(dead_code)]
pub const RIGHT_FACE: Face = Face {interval: [[0, 1, 1], [0, 1, 0], [0, 0, 0], [0, 0, 1]]}; #[allow(dead_code)]
pub const TOP_FACE: Face = Face {interval: [[0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0]]}; #[allow(dead_code)]
pub const BOTTOM_FACE: Face = Face {interval: [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]]}; #[allow(dead_code)]

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
        self.quads.push(Quad {
            coords: [
                [(face.coordinate[0] + face.face.interval[0][0]) as f32, (face.coordinate[1] + face.face.interval[0][1]) as f32, (face.coordinate[2] + face.face.interval[0][2]) as f32],
                [(face.coordinate[0] + face.face.interval[1][0]) as f32, (face.coordinate[1] + face.face.interval[1][1]) as f32, (face.coordinate[2] + face.face.interval[1][2]) as f32],
                [(face.coordinate[0] + face.face.interval[2][0]) as f32, (face.coordinate[1] + face.face.interval[2][1]) as f32, (face.coordinate[2] + face.face.interval[2][2]) as f32],
                [(face.coordinate[0] + face.face.interval[3][0]) as f32, (face.coordinate[1] + face.face.interval[3][1]) as f32, (face.coordinate[2] + face.face.interval[3][2]) as f32],
            ],
            text_coords: get_block(face.blocktype).texture.to_usable(),
        })
    }
}

pub struct Face {
    interval: [[u32; 3]; 4],
}

pub struct MeshFace {
    pub coordinate: [u32; 3],
    pub face: Face, // Which face of block
    pub blocktype: BlockID,
}