use std::fmt::Display;

use crate::{hit::Hit, ray::Ray, Vec3};

use super::Shape;
#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    radius_squared: f64,
    inverse_radius: f64,
}
#[allow(dead_code)]
impl Sphere {
    pub fn new<
        T: Into<f64> + Clone,
        T2: Into<f64> + Clone,
        T3: Into<f64> + Clone,
        T4: Into<f64> + Clone,
    >(
        x: T,
        y: T2,
        z: T3,
        radius: T4,
    ) -> Self {
        Sphere {
            position: Vec3::new(x.into(), y.into(), z.into()),
            radius: radius.clone().into(),
            radius_squared: radius.clone().into().powi(2),
            inverse_radius: 1. / radius.into(),
        }
    }

    pub fn with_position(mut self, position: (f64, f64, f64)) -> Self {
        self.position = Vec3::new(position.0, position.1, position.2);
        self
    }
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    fn get_distance(&self, ray: &crate::ray::Ray) -> Option<f64> {
        let ray_origin_to_sphere_center = ray.origin - self.position;
        let p1 = ray.direction_unit.dot(&ray_origin_to_sphere_center);
        let discriminant =
            (p1).powi(2) - ray_origin_to_sphere_center.magnitude().powi(2) + self.radius_squared;
        if discriminant >= 0f64 {
            Some(-p1 - discriminant.sqrt())
        } else {
            None
        }
    }

    pub fn get_normal_at_point(&self, point: Vec3) -> Vec3 {
        (point - self.position) * self.inverse_radius
    }
    pub fn get_parallel_to_surface_at_point(&self, point: Vec3) -> Vec3 {
        let normal = point - self.position;
        if normal.x > 0.00001 {
            Vec3::new(-(normal.y + normal.z) / normal.x, 1., 1.).normalize()
        } else {
            Vec3::new(1., -(normal.x + normal.z) / normal.y, 1.).normalize()
        }
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            "Sphere at: ".to_string()
                + self.position.to_string().as_str()
                + ", radius: "
                + self.radius.to_string().as_str()
        )
    }
}
impl Shape for Sphere {
    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        let potential_hit = self.get_distance(ray);

        if let Some(distance) = potential_hit {
            let position = ray.at(distance);
            return Some(Hit {
                distance,
                position,
                normal: self.get_normal_at_point(position),
                parallel_to_surface: self.get_parallel_to_surface_at_point(position),
            });
        }
        None
    }
    fn pre_compute(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::{ray::Ray, Vec3};

    #[test]
    fn test_hit() {
        let ray = Ray {
            origin: Vec3::new(0f64, 0f64, 0f64),
            direction_unit: Vec3::new(1f64, 0f64, 0f64),
        };
        let sphere_1 = Sphere::new(10.0, 0.0, 0.0, 20.0);
        let sphere_2 = Sphere::new(10.0, 19.99, 0.0, 20.0);
        let sphere_3 = Sphere::new(10.0, 21.0, 0.0, 20.0);
        assert!(sphere_1.get_distance(&ray).is_some());
        assert!(sphere_2.get_distance(&ray).is_some());
        assert!(sphere_3.get_distance(&ray).is_none());
    }
}
