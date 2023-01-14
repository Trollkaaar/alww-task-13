use crate::utilities::{rays::ray::Ray, vector3::vector::Vec3};

use super::hitRecord::Hit_record;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Hit_record) -> bool {
        false
    }
}
pub trait Material {
    fn scatter(&self, r: &Ray, rec: &Hit_record, attenuation: &Vec3, scattered: &Ray) -> bool {
        false
    }
}
