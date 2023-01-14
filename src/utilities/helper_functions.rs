use std::f32::consts::PI;

use rand::Rng;

use super::{
    objects::{
        hitRecord::Hit_record, hittable_list::Hittable_objects_list, materials::scatter,
        traits::Hittable,
    },
    rays::ray::Ray,
    vector3::{traits::Vector, vector::Vec3},
};

type Color = Vec3;
type Point3 = Vec3;

pub fn write_color(pixel_color: Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;

    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen();
    x
}

pub fn random_float_in_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let range = min..max;
    let x: f32 = rng.gen_range(range);
    x
}

pub fn ray_color(r: &Ray, world: &Hittable_objects_list, depth: u32) -> Vec3 {
    let mut rec = Hit_record::default();

    if (depth <= 0) {
        return Vec3::default();
    }

    if (world.hit(r, 0.001, std::f32::MAX, &mut rec)) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        if scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::default();
        }
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if (x < min) {
        return min;
    }
    if (x > max) {
        return max;
    }
    x
}
