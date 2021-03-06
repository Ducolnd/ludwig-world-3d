use crate::render::{
    vertexarray::VertexArray,
    shapes::shapes::Quad,
    meshing::meshing::*,
};
use crate::world::{
    chunk::chunk::{Chunk, index_to_coord},
    chunk::chunkmanager::ChunkManager,
    constants::*,
    block::blocks::{get_block, BlockID, Blocks, Block},
    chunk::pos::*,
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

            let b = chunk.at_coord_bounds(ChunkCoord {x: x as i16, y: y as i16, z: z as i16});
    
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
    /// Creates a culled mesh. Faces that are not adjecent to a transparent
    /// will not be added to the mesh buffer
    pub fn create_simple_mesh(&mut self, chunk: &Chunk, chunk_manager: &ChunkManager) {

        let mut mesh = Mesh::new();

        for i in 0..(CHUNKSIZE * CHUNKSIZE * WORLDHEIGHT) {
            let (x, y, z) = index_to_coord(i);

            let coord = ChunkCoord {x: x as i16, y: y as i16, z: z as i16};

            let blockid = chunk.at_coord(coord);
            let block = get_block(blockid);

            if block.transparent {
                continue
            }

            // Left
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {x: coord.x + 1, ..coord}, coord, LEFT_FACE, blockid, chunk_manager);
            // Right
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {x: coord.x - 1, ..coord}, coord, RIGHT_FACE, blockid, chunk_manager);
            // Top
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {y: coord.y + 1, ..coord}, coord, TOP_FACE, blockid, chunk_manager);
            // Bottom
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {y: coord.y - 1, ..coord}, coord, BOTTOM_FACE, blockid, chunk_manager);
            // Back
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {z: coord.z + 1, ..coord}, coord, BACK_FACE, blockid, chunk_manager);
            // Front
            ChunkMesh::add_if_needed(chunk, &mut mesh, ChunkCoord {z: coord.z - 1, ..coord}, coord, FRONT_FACE, blockid, chunk_manager);        
        }

        self.mesh = mesh;
    }

    fn add_if_needed(
        chunk: &Chunk,
        mesh: &mut Mesh,
        neighbor_block: ChunkCoord,
        coord: ChunkCoord,
        face: Face,
        block: BlockID,
        manager: &ChunkManager,
    ) {

        let blockid: BlockID;

        // If in bounds, get just get it from the current chunk (faster)
        if Chunk::in_bounds(neighbor_block) {
            blockid = chunk.at_coord(neighbor_block);

        // If not in bounds, request the block from the chunkmanager
        } else {
            // println!("Block {:?} in chunk {:?}", neighbor_block, chunk.pos);
            blockid = manager.get_block_at_coord(WorldCoord::from_chunk_pos(chunk.pos, neighbor_block)).unwrap_or(Blocks::AIR as BlockID)
        }
        
        if get_block(blockid).transparent {
            mesh.add_face(MeshFace {
                coordinate: [coord.x as u32, coord.y as u32, coord.z as u32],
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
        // Todo implement this
    }
}