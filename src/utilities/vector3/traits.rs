use super::vector::Vec3;

pub trait Vector {
    fn dot(&self, rhs: &Self) -> f32;
    fn cross(&self, rhs: Self) -> Vec3;
    fn normalize(&mut self);
    fn unit_vector(&self) -> Vec3;
    fn magnitude(&self) -> f32;
    fn magnitude_squared(&self) -> f32;
}
