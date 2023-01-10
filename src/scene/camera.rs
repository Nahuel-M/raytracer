use nalgebra::{Vector3, Rotation, Rotation3};

use crate::ray::Ray;

pub struct Camera {
    pub location: Vector3<f64>,
    pub rotation: Rotation3<f64>,
    pixel_size : f64,
}

impl Camera {
    pub fn new(fov_radians_horizontal: f64, pixel_count_horizontal : u32) -> Self {
        
        let pixel_size = (fov_radians_horizontal / 2.0).tan() / (pixel_count_horizontal as f64 / 2.0);
        Camera {
            location: Vector3::new(0., 0., 0.),
            rotation: Rotation3::new(Vector3::new(0., 0., 0.)),
            pixel_size
        }
    }
    pub fn with_pixel_size(location : Vector3<f64>, rotation : Vector3<f64>, pixel_size : f64) -> Self{
        Camera {
            location,
            rotation: Rotation3::new(rotation),
            pixel_size,
        }
    }
    pub fn ray_for_pixel(&self, x : i32, y: i32) -> Ray{
        
        let pixel_normal = 
        
        self.rotation.transform_vector(
            &Vector3::new(
            x as f64 * self.pixel_size,
            -y as f64 * self.pixel_size,
            1f64)
        )
        .normalize();
        Ray{ origin: self.location, direction_unit: pixel_normal }
    }
}
