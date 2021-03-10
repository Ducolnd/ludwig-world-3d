use cgmath::{Point3};

use crate::render::camera::Camera;
use crate::render::low::master::Master;

pub struct Player {
    pub position: Point3<f32>,
}

impl Player {
    pub fn new(at: Point3<f32>) -> Self {
        Self {
            position: at,
        }
    }

    pub fn null_player() -> Self {
        Self {
            position: (0.0, 0.0, 0.0).into(),
        }
    }
}