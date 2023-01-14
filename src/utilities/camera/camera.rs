use crate::utilities::{
    rays::ray::Ray,
    vector3::{traits::Vector, vector::Vec3},
};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, img_width: i32) -> Self {
        let img_height = (img_width as f32 / aspect_ratio) as i32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let _origin = Vec3::new(0.0, 0.0, 0.0);
        let _horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let _vertical = Vec3::new(0.0, viewport_height, 0.0);
        let _lower_left_corner =
            _origin - 0.5 * _horizontal - 0.5 * _vertical - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin: _origin,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner: _lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
