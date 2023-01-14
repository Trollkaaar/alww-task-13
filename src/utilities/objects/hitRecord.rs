use crate::utilities::{
    rays::ray::Ray,
    vector3::{traits::Vector, vector::Vec3},
};

use super::materials::Material;
#[derive(Default)]
pub struct Hit_record {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl Hit_record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        if (r.direction.dot(&outward_normal) > 0.0) {
            self.n = -1.0 * outward_normal;
            self.front_face = false;
        } else {
            self.n = outward_normal;
            self.front_face = true;
        }
    }
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn n(&self) -> Vec3 {
        self.n
    }
}
