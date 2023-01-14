use crate::utilities::vector3::{traits::Vector, vector::Vec3};

use super::{hittable_list::Hittable_objects_list, materials::Material, traits::Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::utilities::rays::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut super::hitRecord::Hit_record,
    ) -> bool {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = (r.at(rec.t()));
                rec.set_face_normal(r, (rec.p - self.center) / self.radius);
                rec.material = self.material;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = (temp);
                rec.p = (r.at(rec.t()));
                rec.set_face_normal(r, (rec.p - self.center) / self.radius);
                rec.material = self.material;
                return true;
            }
        }
        return false;
    }
}
