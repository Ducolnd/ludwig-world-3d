use std::collections::HashMap;
use std::time::Instant;

use crate::world::chunk::{chunk::Chunk, pos::*};
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::render::low::master::Master;

/// Takes care of loading chunks, meshing chunks, unloading chunks
pub struct ChunkManager {
    loaded_chunks: HashMap<ChunkPos, Chunk>,
    chunks_meshes: HashMap<ChunkPos, ChunkMesh>,

    render_distance: u32,

    chunk_meshing_time: u128,
    chunk_loading_time: u128,
}

impl ChunkManager {
    pub fn init(render_distance: u32) -> Self {
        let loaded_chunks = HashMap::new();
        let chunks_meshes = HashMap::new();

        Self {
            loaded_chunks,
            chunks_meshes,

            render_distance,
            chunk_meshing_time: 0,
            chunk_loading_time: 0,
        }
    }

    fn load_chunk(&mut self, pos: ChunkPos, master: &mut Master) {
        let mut chunk = Chunk::new(pos);

        let now = Instant::now();
        chunk.generate();
        let lapsed = now.elapsed();

        let mut mesh = ChunkMesh::new();

        let now = Instant::now();
        mesh.create_simple_mesh(&chunk, self.get_neighbors(pos));
        let elapsed = now.elapsed();

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

        self.chunk_meshing_time += elapsed.as_micros();
        self.chunk_loading_time += lapsed.as_micros();
    }

    pub fn center_around(&mut self, center: ChunkPos, master: &mut Master) {
        for x in -1 * self.render_distance as i32..(self.render_distance + 1) as i32 {
            for z in -1 * self.render_distance as i32..(self.render_distance + 1) as i32 {
                self.load_chunk(center + ChunkPos::new(x, 0, z), master);
            }
        }

        println!("Loaded {} chunks", self.loaded_chunks.len());
    }

    /// Returns chunks around a given chunk in this order:
    /// [U, R, D, L]
    pub fn get_neighbors(&self, center: ChunkPos) -> [Option<&Chunk>; 4]{
        [
            self.get_chunk_option(center + ChunkPos::new(0, 0, 1)),
            self.get_chunk_option(center + ChunkPos::new(1, 0, 0)),
            self.get_chunk_option(center + ChunkPos::new(0, 0, -1)),
            self.get_chunk_option(center + ChunkPos::new(-1, 0, 0)),
        ]
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

    pub fn get_chunk_option(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.loaded_chunks.get(&pos)
    }

    // Some timing stuff
    pub fn meshing_time(&self) -> u128 {
        self.chunk_meshing_time / self.chunks_meshes.len() as u128
    }

    pub fn loading_time(&self) -> u128 {
        self.chunk_loading_time / self.chunks_meshes.len() as u128
    }
}