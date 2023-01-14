use crate::utilities::{
    rays::ray::Ray,
    vector3::{traits::Vector, vector::Vec3},
};

use super::hitRecord::Hit_record;
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
    Dialectric {},
}
impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    r: &Ray,
    rec: &Hit_record,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p + rec.n + Vec3::random_in_unit_sphere();
            *scattered = Ray::new(rec.p, target - rec.p);
            *attenuation = albedo;

            return true;
        }
        &Material::Metal { albedo } => {
            let reflected = reflect(&r.direction.unit_vector(), &rec.n);
            *scattered = Ray::new(rec.p, reflected);
            *attenuation = albedo;
            return scattered.direction.dot(&rec.n) > 0.0;
        }
        &Material::Dialectric {} => false,
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    //This shit janky as hell
    *v - (2.0 * v.dot(&n)) * *n
}
