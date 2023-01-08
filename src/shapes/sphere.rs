use nalgebra::Vector3;

use crate::{ray::Ray, renderer::RayRenderable};

pub struct Sphere {
    pub(crate) location: Vector3<f64>,
    pub radius: f64,
    radius_squared : f64,
}

impl Sphere {
    pub fn new(x : f64, y : f64, z : f64, radius : f64) -> Self{
        Sphere{ location: Vector3::new(x, y, z), radius, radius_squared : radius.powi(2) }
    }
}

impl RayRenderable for Sphere{
    fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let ray_origin_to_sphere_center = ray.origin - self.location;
        let p1 = ray.direction_unit.dot(&ray_origin_to_sphere_center);
        let discriminant = (p1).powi(2) - ray_origin_to_sphere_center.magnitude().powi(2) + self.radius_squared;
        if discriminant >= 0f64 {
            Some(- p1 - discriminant.sqrt())
        } else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::{ray::Ray, renderer::RayRenderable};
    use nalgebra::Vector3;

    #[test]
    fn test_hit() {
        let ray = Ray {
            origin: Vector3::new(0f64, 0f64, 0f64),
            direction_unit: Vector3::new(1f64, 0f64, 0f64),
        };
        let sphere_1 = Sphere::new(10.0, 0.0, 0.0, 20.0);
        let sphere_2 = Sphere::new(10.0, 19.99, 0.0, 20.0);      
        let sphere_3 = Sphere::new(10.0, 21.0, 0.0, 20.0);
        assert!(sphere_1.get_hit_distance(&ray).is_some());
        assert!(sphere_2.get_hit_distance(&ray).is_some());
        assert!(sphere_3.get_hit_distance(&ray).is_none());
    }
}
