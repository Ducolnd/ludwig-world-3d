use std::collections::HashMap;
use std::time::Instant;

use crate::render::low::master::Master;
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::world::chunk::chunk::{Chunk};
use crate::world::chunk::pos::*;

pub struct World {
    loaded_chunks: HashMap<ChunkPos, Chunk>,
    chunks_meshes: HashMap<ChunkPos, ChunkMesh>,
}

impl World {
    pub fn new() -> Self {
        let loaded_chunks = HashMap::new();
        let chunks_meshes = HashMap::new();

        Self {
            loaded_chunks, 
            chunks_meshes
        }
    }

    pub fn load_chunk(&mut self, pos: ChunkPos, master: &mut Master) {
        let mut chunk = Chunk::new(pos);
        chunk.generate();

        let mut mesh = ChunkMesh::new();

        // Create mesh and time it
        let now = Instant::now();
        mesh.create_simple_mesh(&chunk);
        let elapsed = now.elapsed();
        println!("It took {} microseconds to mesh this chunk. This chunk has {} quads", elapsed.as_micros(), mesh.mesh.quads.objects.len());

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

    /// Update the master buffers
    pub fn update_chunk_buffer(&self, master: &mut Master, pos: ChunkPos) {

        match Some(self.chunks_meshes.get(&pos)) {
            None => println!("ERROR: Chunkpos was not loaded!"),
            Some(chunkmesh) => master.add_to_buffer(&chunkmesh.unwrap().to_vertex_array(), pos),
        }
    }
}