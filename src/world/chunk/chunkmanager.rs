use std::collections::HashMap;
use std::time::Instant;

use crate::world::{
    chunk::{chunk::Chunk, pos::*},
    constants::*,
    block::blocks::BlockID,
    world::World,
};
use crate::render::{
    low::{
        renderer::Renderer,
        context::Context,
    },
    meshing::chunkmeshing::ChunkMesh,
    drawables::chunk::ChunkDrawable,
};

/// Takes care of loading chunks, meshing chunks, unloading chunks
pub struct ChunkManager {
    loaded_chunks: HashMap<ChunkPos, Chunk>,
    chunks_meshes: HashMap<ChunkPos, ChunkMesh>,
    /// The buffers used for rendering
    pub chunk_buffers: HashMap<ChunkPos, ChunkDrawable>,
    load_queue: Vec<ChunkPos>,

    updated: bool,

    chunk_meshing_time: u128,
    chunk_loading_time: u128,
}

impl ChunkManager {
    pub fn new(render_distance: u32) -> Self {
        let loaded_chunks = HashMap::new();
        let chunks_meshes = HashMap::new();
        let chunk_buffers = HashMap::new();

        Self {
            loaded_chunks,
            chunks_meshes,
            chunk_buffers,
            load_queue: vec![],

            updated: false,

            chunk_meshing_time: 1,
            chunk_loading_time: 1,
        }
    }

    pub fn set_camera_location(&mut self, coord: WorldCoord) {
        let chunkpos = coord.to_chunk_coord();

        // If center chunk is not yet loaded
        if !self.loaded_chunks.contains_key(&chunkpos) {
            println!("A new chunk is the center");
            self.updated = false;
            self.unload_chunk();
            self.queue_chunk_load(chunkpos);
        }
    }

    /// Loads and meshes a single chunks
    pub fn load_chunk(&mut self, pos: ChunkPos, height: [u32; CHUNKSIZE * CHUNKSIZE], renderer: &mut Renderer) {
        let mut chunk = Chunk::new(pos);

        renderer.chunkpos_uniform.add(&renderer.queue, pos, pos.to_raw());

        let now = Instant::now();
        chunk.generate(height);
        let lapsed = now.elapsed();

        self.loaded_chunks.insert(
            pos,
            chunk,
        );
        self.chunk_loading_time += lapsed.as_micros();

        self.mesh_neighbors(pos);
    }

    pub fn mesh_neighbors(&mut self, pos: ChunkPos) {
        self.mesh_chunk(pos);

        self.mesh_chunk(ChunkPos {x: pos.x + 1, ..pos});
        self.mesh_chunk(ChunkPos {x: pos.x - 1, ..pos});
        self.mesh_chunk(ChunkPos {z: pos.z + 1, ..pos});
        self.mesh_chunk(ChunkPos {z: pos.z - 1, ..pos});
    }

    /// Mesh a single chunk. Does nothing if pos is not loaded
    pub fn mesh_chunk(&mut self, pos: ChunkPos) {
        
        let c = &self.loaded_chunks.get(&pos);
        if !c.is_none() {
            let mut mesh = ChunkMesh::new();

            let now = Instant::now();
            mesh.create_simple_mesh(c.unwrap(), &self);
            let elapsed = now.elapsed();

            self.chunks_meshes.insert(
                pos, 
                mesh
            );

            self.chunk_meshing_time += elapsed.as_micros();
        }
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

    // Queue a chunk for loading
    pub fn queue_chunk_load(&mut self, pos: ChunkPos) {
        self.load_queue.push(pos);
    }

    /// Load and mesh all chunks in queue
    pub fn load_queue(&mut self, world: &World, renderer: &mut Renderer) {
        if self.load_queue.len() > 0 {
            for pos in self.load_queue.clone() {
                self.load_chunk(pos.clone(), world.map.create_heightmap(&pos), renderer);
            }
    
            self.load_queue.clear();
        }
    }

    /// Doesn't actually remove data from buffer at this point
    /// so chunks will still be rendered. ToDo implement this
    pub fn unload_chunk(&mut self) {
        self.chunks_meshes.clear();
        self.loaded_chunks.clear();
        self.chunk_buffers.clear();
    }

    /// A low level function that updates the buffers according to the meshes for rendering
    pub fn update(&mut self, context: &mut Context, encoder: &mut wgpu::CommandEncoder) {
        if !self.updated {
            self.updated = true;
            for (pos, chunk) in &self.chunks_meshes {
                let mut c = ChunkDrawable::new(&context.renderer.device, *pos);
                c.from_chunk_mesh(&chunk, &context.renderer.device, encoder);

                self.chunk_buffers.insert(*pos, c);
            }
        }
    }

    /// Panics if pos is not loaded
    pub fn get_chunk(&self, pos: ChunkPos) -> &Chunk {
        self.loaded_chunks.get(&pos).unwrap()
    }

    pub fn get_chunk_option(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.loaded_chunks.get(&pos)
    }

    pub fn get_mesh(&self, pos: ChunkPos) -> &ChunkMesh {
        self.chunks_meshes.get(&pos).unwrap()
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