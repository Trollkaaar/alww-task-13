use rand::Rng;

use crate::utilities::vector3::traits::*;
use crate::{random_float, utilities::helper_functions::random_float_in_range};
use std::ops::{self, Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        Self {
            x: _x,
            y: _y,
            z: _z,
        }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_float_in_range(min, max),
            random_float_in_range(min, max),
            random_float_in_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::default();

        loop {
            p = Vec3::random_in_range(-1.0, 1.0);

            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}
impl Copy for Vec3 {}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Vector for Vec3 {
    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    fn cross(&self, rhs: Self) -> Vec3 {
        Self {
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.x - self.x * rhs.z),
            z: (self.x * rhs.z - self.y * rhs.x),
        }
    }

    fn normalize(&mut self) {
        let mag: f32 = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    fn unit_vector(&self) -> Vec3 {
        let mag: f32 = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn magnitude(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)).sqrt()
    }
    fn magnitude_squared(&self) -> f32 {
        (self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)
    }
}
