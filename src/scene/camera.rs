use crate::{
    algebra::{quaternion::Quaternion},
    ray::Ray,
    Vec3,
};
#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation_quaternion: Quaternion,
    pub pixel_size: f64,
}
#[allow(dead_code)]
impl Camera {
    pub fn new(fov_radians_horizontal: f64, pixel_count_horizontal: u32) -> Self {
        let pixel_size =
            (fov_radians_horizontal / 2.0).tan() / (pixel_count_horizontal as f64 / 2.0);
        Camera {
            position: Vec3::new(0., 0., 0.),
            rotation_quaternion: Quaternion::new(),
            pixel_size,
        }
    }

    pub fn with_pixel_size(location: Vec3, pixel_size: f64) -> Self {
        Camera {
            position: location,
            rotation_quaternion: Quaternion::new(),
            pixel_size,
        }
    }
    pub fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let mut pixel_normal =
            Vec3::new(x * self.pixel_size, -y * self.pixel_size, -1f64).normalize();
        // self.rotation_quaternion.rotate_vector(&mut pixel_normal);
        Ray {
            origin: self.position,
            direction_unit: pixel_normal,
        }
    }
    pub fn look_at(&mut self, position: Vec3) {
        self.rotation_quaternion =
            Quaternion::from_unit_vectors(Vec3::z(), (position - self.position).normalize());
    }
}
