use crate::render::meshing::meshing::*;
use crate::render::{
    vertexarray::VertexArray,
    shapes::shapes::Quad,
};
use crate::world::{
    chunk::chunk::{Chunk, index_to_coord},
    constants::*,
    block::blocks::{get_block, BlockID},
};

pub struct ChunkMesh {
    pub mesh: Mesh, // ToDo make private
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {mesh: Mesh::new()}
    }

    pub fn to_vertex_array(&self) -> &VertexArray<Quad> {
        &self.mesh.quads
    }

    #[allow(dead_code)]
    /// Create mesh with every block representing 6 quads
    pub fn create_dumb_mesh(&mut self, chunk: &Chunk) {

        let mut mesh = Mesh::new();

        for i in 0..(CHUNKSIZE * CHUNKSIZE * WORLDHEIGHT) {
            let (x, y, z) = index_to_coord(i);

            let b = chunk.at_coord(x as i32, y as i32, z as i32);
    
            if get_block(b).transparent {
                continue
            }

            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: RIGHT_FACE,
                blocktype: b,
            });
            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: FRONT_FACE,
                blocktype: b,
            });
            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: BACK_FACE,
                blocktype: b,
            });
            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: LEFT_FACE,
                blocktype: b,
            });
            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: TOP_FACE,
                blocktype: b,
            });
            mesh.add_face(MeshFace {
                coordinate: [x, y, z],
                face: BOTTOM_FACE,
                blocktype: b,
            });
        }

        self.mesh = mesh;
    }

    #[allow(dead_code)]
    /// Creates a culled mesh. Blocks that are not adjecent to a transparent
    /// will not be added to the mesh buffer
    pub fn create_simple_mesh(&mut self, chunk: &Chunk) {

        let mut mesh = Mesh::new();

        for i in 0..(CHUNKSIZE * CHUNKSIZE * WORLDHEIGHT) {
            let (x, y, z) = index_to_coord(i);

            let x = x as i32;
            let y = y as i32;
            let z = z as i32;

            let blockid = chunk.at_coord(x as i32, y as i32, z as i32);
            let block = get_block(blockid);

            if block.transparent {
                continue
            }

            // Left
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x + 1, y, z], [x, y, z], LEFT_FACE, blockid);
            // Right
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x - 1, y, z], [x, y, z], RIGHT_FACE, blockid);
            // Top
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x, y + 1, z], [x, y, z], TOP_FACE, blockid);
            // Bottom
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x, y - 1, z], [x, y, z], BOTTOM_FACE, blockid);
            // Back
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x, y, z + 1], [x, y, z], BACK_FACE, blockid);
            // Front
            ChunkMesh::add_if_needed(chunk, &mut mesh, [x, y, z - 1], [x, y, z], FRONT_FACE, blockid);        
        }

        self.mesh = mesh;
    }

    fn add_if_needed(
        chunk: &Chunk,
        mesh: &mut Mesh,
        check_at: [i32; 3],
        coord: [i32; 3],
        face: Face,
        block: BlockID,

    ) {
        if get_block(chunk.at_coord(check_at[0], check_at[1], check_at[2])).transparent {
            mesh.add_face(MeshFace {
                coordinate: [coord[0] as u32, coord[1] as u32, coord[2] as u32],
                face: face,
                blocktype: block,
            });
        }
    }

    #[allow(dead_code)]
    /// Create an optimized mesh where all adjecent block of the same type are
    /// formed into a single quad. This is only useful if GPU memory usage is high
    /// because generating this mesh is slower
    pub fn create_greedy_mesh() {
    }
}