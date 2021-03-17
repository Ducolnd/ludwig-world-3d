use crate::render::shapes::shape::Shape;
use crate::render::low::vertex::Vertex;

pub struct Quad {
    pub coords: [[f32; 3]; 4],
    pub text_coords: [[f32; 2]; 4],
}

impl Shape for Quad {
    fn vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex { position: [self.coords[0][0], self.coords[0][1], self.coords[0][2]], text_coords: self.text_coords[0] },
            Vertex { position: [self.coords[1][0], self.coords[1][1], self.coords[1][2]], text_coords: self.text_coords[1] },
            Vertex { position: [self.coords[2][0], self.coords[2][1], self.coords[2][2]], text_coords: self.text_coords[2] },
            Vertex { position: [self.coords[3][0], self.coords[3][1], self.coords[3][2]], text_coords: self.text_coords[3] },
        ]
    }
    
    fn indexes(&self, offset: u32) -> Vec<u32> {
        vec![
            0 + offset, 1 + offset, 2 + offset,
            2 + offset, 3 + offset, 0 + offset,
        ]
    }

    fn num_indices() -> usize {
        6
    }

    fn num_vertices() -> usize {
        4
    }
}

pub struct Triangle {
    pub coords: [[f32; 2]; 3],
    pub colors: [f32; 2],
}

impl Shape for Triangle {
    fn vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex { position: [self.coords[0][0], self.coords[0][1], 0.0], text_coords: self.colors },
            Vertex { position: [self.coords[1][0], self.coords[1][1], 0.0], text_coords: self.colors },
            Vertex { position: [self.coords[2][0], self.coords[2][1], 0.0], text_coords: self.colors },
        ]
    }
    
    fn indexes(&self, offset: u32) -> Vec<u32> {
        vec![
            0 + offset, 1 + offset, 2 + offset,
        ]
    }

    fn num_indices() -> usize {
        3
    }

    fn num_vertices() -> usize {
        3
    }
}