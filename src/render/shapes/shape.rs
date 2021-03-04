use crate::render::low::vertex::Vertex;

pub trait Shape {
    fn num_indices() -> usize;
    fn num_vertices() -> usize;
    fn vertices(&self) -> Vec<Vertex>;
    fn indexes(&self, offset: u32) -> Vec<u32>;
}
