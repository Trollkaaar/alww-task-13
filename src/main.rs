mod utilities;
use rand::Rng;
use std::f32::consts::PI;

use utilities::{
    objects::{
        hitRecord::{self, Hit_record},
        traits::Hittable,
    },
    rays::ray::Ray,
    vector3::{traits::Vector, vector::Vec3},
};

use crate::utilities::{
    camera::camera::Camera,
    helper_functions::{random_float, random_float_in_range, ray_color, write_color},
    objects::{hittable_list::Hittable_objects_list, materials::Material, sphere::Sphere},
};
type Color = Vec3;
type Point3 = Vec3;
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 1600;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 1000;
    let max_depth = 100;

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(
        (Point3::new(0.0, 0.0, -1.0)),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.3, 0.3),
        },
    )));
    list.push(Box::new(Sphere::new(
        (Point3::new(0.0, -100.5, -1.0)),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    )));
    list.push(Box::new(Sphere::new(
        (Point3::new(1.0, 0.0, -1.0)),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
        },
    )));
    list.push(Box::new(Sphere::new(
        (Point3::new(-1.0, 0.0, -1.0)),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
        },
    )));

    let world = Hittable_objects_list::new(list);

    let cam = Camera::new(aspect_ratio, img_width);

    println!("P3\n{} {}\n255", img_width, img_height);

    let mut progress: u8 = 0;
    for j in (0..img_height).rev() {
        let _progress = ((100.0 / img_height as f32) * (img_height as f32 - j as f32)) as u8;
        if (_progress != progress) {
            progress = _progress;
            eprintln!("Progress: {}%", progress);
        }
        for i in 0..img_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f32 + random_float()) / (img_width - 1) as f32;
                let v = (j as f32 + random_float()) / (img_height - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
}
