use crate::utilities::vector3::{traits::Vector, vector::Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(_origin: Vec3, _direction: Vec3) -> Self {
        Self {
            origin: _origin,
            direction: _direction,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
