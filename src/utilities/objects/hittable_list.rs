use crate::utilities::vector3::{traits::Vector, vector::Vec3};

use super::{hitRecord::Hit_record, traits::Hittable};

pub struct Hittable_objects_list {
    list_of_objects: Vec<Box<dyn Hittable>>,
}

impl Hittable_objects_list {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Hittable_objects_list {
        Hittable_objects_list {
            list_of_objects: list,
        }
    }
}

impl Hittable for Hittable_objects_list {
    fn hit(
        &self,
        r: &crate::utilities::rays::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut Hit_record,
    ) -> bool {
        let mut tmp_rec = Hit_record::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for object in &self.list_of_objects {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t();
                rec.t = tmp_rec.t();
                rec.p = tmp_rec.p();
                rec.n = tmp_rec.n();
                rec.material = tmp_rec.material;
            }
        }
        hit_anything
    }
}
