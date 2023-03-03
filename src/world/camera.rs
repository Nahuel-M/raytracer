use std::fmt::Display;

use image::RgbImage;

use crate::{
    algebra::{quaternion::Quaternion},
    algebra::ray::Ray,
    Vec3,
};
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub rotation_quaternion: Quaternion,
    pub pixel_size: f64,
    image_size : (u32, u32),
}
#[allow(dead_code)]
impl Camera {
    pub fn new(fov_radians_horizontal: f64, image : &RgbImage) -> Self {
        let pixel_size =
            (fov_radians_horizontal / 2.0).tan() / (image.width() as f64 / 2.0);
        Camera {
            position: Vec3::new(0., 0., 0.),
            rotation_quaternion: Quaternion::new(),
            pixel_size,
            image_size : (image.width(), image.height())
        }
    }

    // pub fn with_pixel_size(location: Vec3, pixel_size: f64) -> Self {
    //     Camera {
    //         position: location,
    //         rotation_quaternion: Quaternion::new(),
    //         pixel_size,
    //     }
    // }
    pub fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let mut pixel_normal =
            Vec3::new(
                (x - self.image_size.0 as f64 / 2.) * self.pixel_size, 
                -(y - self.image_size.1 as f64 / 2.) * self.pixel_size, 
                1f64
            )
            .normalize();
        self.rotation_quaternion.rotate_vector(&mut pixel_normal);
        Ray {
            origin: self.position,
            direction_unit: pixel_normal,
        }
    }
    pub fn look_at(&mut self, position: Vec3) {
        let up = Vec3::Y;
        let direction = (position - self.position).normalize();
        let v = direction - up * up.dot(&direction);
        let q = Quaternion::from_unit_vectors(&Vec3::Z, &v);
        self.rotation_quaternion =
            Quaternion::from_unit_vectors(&v, &direction) * q;
    }
}

impl Display for Camera{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera at {} with orientation quaternion {}", self.position, self.rotation_quaternion)
    }
}