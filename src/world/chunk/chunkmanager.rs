use std::collections::HashMap;
use std::time::Instant;

use crate::world::chunk::{chunk::Chunk, pos::*};
use crate::world::map::Map;
use crate::world::block::blocks::BlockID;
use crate::render::meshing::chunkmeshing::ChunkMesh;
use crate::render::low::master::Master;

/// Takes care of loading chunks, meshing chunks, unloading chunks
pub struct ChunkManager {
    pub loaded_chunks: HashMap<ChunkPos, Chunk>,
    chunks_meshes: HashMap<ChunkPos, ChunkMesh>,

    pub render_distance: u32,

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
            chunk_meshing_time: 1,
            chunk_loading_time: 1,
        }
    }

    pub fn load_chunk(&mut self, pos: ChunkPos, master: &mut Master, height: u32) {
        println!("loading chunk at : {:?}", pos);

        let mut chunk = Chunk::new(pos);

        let now = Instant::now();
        chunk.generate(height);
        let lapsed = now.elapsed();

        self.loaded_chunks.insert(
            pos,
            chunk,
        );

        self.chunk_loading_time += lapsed.as_micros();

        self.mesh_neighbors(master, pos);
    }

    pub fn mesh_neighbors(&mut self, master: &mut Master, pos: ChunkPos) {
        self.mesh_chunk(master, pos);

        self.mesh_chunk(master, ChunkPos {x: pos.x + 1, ..pos});
        self.mesh_chunk(master, ChunkPos {x: pos.x - 1, ..pos});
        self.mesh_chunk(master, ChunkPos {z: pos.z + 1, ..pos});
        self.mesh_chunk(master, ChunkPos {z: pos.z - 1, ..pos});
    }

    /// Mesh a single chunk. Panics if pos is not loaded
    pub fn mesh_chunk(&mut self, master: &mut Master, pos: ChunkPos) {
        
        let c = &self.loaded_chunks.get(&pos);
        if !c.is_none() {
            let mut mesh = ChunkMesh::new();

            let now = Instant::now();
            mesh.create_simple_mesh(c.unwrap(), &self);
            let elapsed = now.elapsed();

            master.chunkpos_uniform.add(&master.queue, pos, pos.to_raw());
            // Upload mesh to GPU
            master.new_buffer(mesh.to_vertex_array(), pos);

            self.chunks_meshes.insert(
                pos, 
                mesh
            );

            self.chunk_meshing_time += elapsed.as_micros();
        }
        
    }

    pub fn center_around(&mut self, center: ChunkPos, master: &mut Master, map: &Map) {
        for x in -1 * self.render_distance as i32..(self.render_distance + 1) as i32 {
            for z in -1 * self.render_distance as i32..(self.render_distance + 1) as i32 {
                self.load_chunk(center + ChunkPos::new(x, 0, z), master, 10);
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

    /// Get the block at the given coord. Returns an option
    pub fn get_block_at_coord(&self, coord: WorldCoord) -> Option<BlockID> {
        if let Some(chunk) = self.loaded_chunks.get(&coord.to_chunk_coord()) {
            return Some(chunk.at_coord(coord.to_chunk_local()))
        } 
        
        else {
            return None
        }
    }

    // Some timing stuff
    pub fn meshing_time(&self) -> u128 {
        self.chunk_meshing_time / self.chunks_meshes.len() as u128
    }

    pub fn loading_time(&self) -> u128 {
        self.chunk_loading_time / self.chunks_meshes.len() as u128
    }
}