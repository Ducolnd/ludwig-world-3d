use crate::render::shapes::shape::Shape;
use crate::render::low::vertex::Vertex;
use cgmath;

// fn vertex_help(point: cgmath::Point3<f32>, array: [[u32; 3]; 4]) -> [[f32; 3]; 4] {
//     [
//         [array[0][0] as f32 + point.x, array[0][1] as f32 + point.y, array[0][2] as f32 + point.z, ],
//         [array[1][0] as f32 + point.x, array[1][1] as f32 + point.y, array[1][2] as f32 + point.z, ],
//         [array[2][0] as f32 + point.x, array[2][1] as f32 + point.y, array[2][2] as f32 + point.z, ],
//         [array[3][0] as f32 + point.x, array[3][1] as f32 + point.y, array[3][2] as f32 + point.z, ],
//     ]
// }

// impl Shape for Square {
//     fn vertices(&self) -> Vec<Vertex> {
//         let mut v = Vec::<Vertex>::with_capacity(4 * 6); // 4 vertices for 6 faces

//         for i in self.quads.iter() {
//             v.append(&mut i.vertices());
//         }

//         v
//     }
    
//     fn indexes(&self, offset: u32) -> Vec<u32> {
//         vec![
//             0 + offset, 1 + offset, 2 + offset, 2 + offset, 3 + offset, 0 + offset, // top
//             4 + offset, 5 + offset, 6 + offset, 6 + offset, 7 + offset, 4 + offset, // bottom
//             8 + offset, 9 + offset, 10 + offset, 10 + offset, 11 + offset, 8 + offset, // right
//             12 + offset, 13 + offset, 14 + offset, 14 + offset, 15 + offset, 12 + offset, // left
//             16 + offset, 17 + offset, 18 + offset, 18 + offset, 19 + offset, 16 + offset, // front
//             20 + offset, 21 + offset, 22 + offset, 22 + offset, 23 + offset, 20 + offset, // back
//         ]
//     }

//     fn num_indices() -> usize {
//         6 * 6
//     }

//     fn num_vertices() -> usize {
//         4 * 6
//     }
// }

#[derive(Debug)]
/// Front side is always first top-right then counterclockwise the rest
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