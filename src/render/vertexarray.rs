use crate::render::{
    low::{
        vertex::Vertex,
    },
    shapes::shape::Shape,
};

pub struct VertexArray<T: Shape> {
    pub objects: Vec<T>,
}

impl<T: Shape> VertexArray<T> {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn push(&mut self, shape: T) {
        self.objects.push(shape);
    }

    pub fn to_indices(&self) -> Vec<u32> {
        let mut v = Vec::<u32>::with_capacity(self.objects.len() * T::num_indices());

        for (i, val) in self.objects.iter().enumerate() {
            v.append(&mut val.indexes((i * T::num_vertices()) as u32));
        }
        
        v
    }

    pub fn to_vertices(&self) -> Vec<Vertex> {
        let mut v = Vec::<Vertex>::with_capacity(self.objects.len() * T::num_vertices());

        for val in self.objects.iter() {
            v.append(
                &mut val.vertices()
            );
        }
        
        v
    }
}