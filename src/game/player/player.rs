use cgmath::{Point3};

pub struct Player {
    pub position: Point3<f32>,
}

impl Player {
    pub fn null_player() -> Self {
        Self {
            position: (0.0, 0.0, 0.0).into(),
        }
    }
}