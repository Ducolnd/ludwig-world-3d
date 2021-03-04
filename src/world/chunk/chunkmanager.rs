use std::collections::HashMap;

use crate::world::chunk::{chunk::Chunk, pos::*};
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::render::low::master::Master;

/// Takes care of loading chunks, meshing chunks, unloading chunks
pub struct ChunkManager {
    loaded_chunks: HashMap<ChunkPos, Chunk>,
    chunks_meshes: HashMap<ChunkPos, ChunkMesh>,

    render_distance: u32,
}

impl ChunkManager {
    pub fn init(render_distance: u32) -> Self {
        let loaded_chunks = HashMap::new();
        let chunks_meshes = HashMap::new();

        Self {
            loaded_chunks,
            chunks_meshes,

            render_distance
        }
    }

    fn load_chunk(&mut self, pos: ChunkPos, master: &mut Master) {
        let mut chunk = Chunk::new(pos);
        chunk.generate();

        let mut mesh = ChunkMesh::new();

        mesh.create_simple_mesh(&chunk);
        master.chunkpos_uniform.add(&master.queue, pos, pos.to_raw());

        // Upload mesh to GPU
        master.new_buffer(mesh.to_vertex_array(), pos);

        self.loaded_chunks.insert(
            pos,
            chunk,
        );

        self.chunks_meshes.insert(
            pos, 
            mesh
        );
    }

    pub fn center_around(&mut self, center: ChunkPos, master: &mut Master) {
        self.load_chunk(center, master);

        // Load 4 chunks around center
        self.load_chunk(center - ChunkPos::new(1, 0, 0), master);
        self.load_chunk(center + ChunkPos::new(1, 0, 0), master);
        self.load_chunk(center - ChunkPos::new(0, 0, 1), master);
        self.load_chunk(center + ChunkPos::new(0, 0, 1), master);
    }

    /// Doesn't actually remove data from buffer at this point
    /// so chunks will still be rendered
    pub fn unload_chunk(&mut self, pos: ChunkPos, master: &mut Master) {
        self.chunks_meshes.remove(&pos);
        self.loaded_chunks.remove(&pos);
    }

    /// Panics if pos is not loaded
    pub fn get_chunk(&self, pos: ChunkPos) -> &Chunk {
        self.loaded_chunks.get(&pos).unwrap()
    }
}