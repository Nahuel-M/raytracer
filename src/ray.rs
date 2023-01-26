use std::f64::consts::PI;

use crate::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction_unit: Vec3,
}

impl Ray {
    #[inline]
    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction_unit * distance
    }

    #[inline]
    pub fn reflect_specular(&self, surface_normal: Vec3, new_origin: Vec3) -> Self {
        let reflected_direction =
            self.direction_unit - 2. * self.direction_unit.dot(&surface_normal) * surface_normal;
        Ray {
            origin: new_origin,
            direction_unit: reflected_direction,
        }
    }

    #[inline]
    pub fn reflect_diffuse(
        &self,
        surface_normal: Vec3,
        mut parallel_to_surface: Vec3,
        new_origin: Vec3,
    ) -> Self {
        let random_angle = fastrand::f64() * 2. * PI;
        parallel_to_surface =
            Ray::rodrigues_rotation(parallel_to_surface, surface_normal, random_angle);
        let random_angle = Ray::random_cosine_distribution();
        let diffuse_direction =
            Ray::rotate_towards_vector(surface_normal, parallel_to_surface, random_angle);
        Ray {
            origin: new_origin,
            direction_unit: diffuse_direction,
        }
    }

    #[inline]
    pub fn refract(&self, mut surface_normal: Vec3, new_origin: Vec3, mut ior: f64) -> Self {
        if surface_normal.dot(&self.direction_unit) < 0. {
            ior = 1. / ior;
        } else {
            surface_normal *= -1.;
        }
        let r_orthogonal = ior
            * (self.direction_unit + (-self.direction_unit.dot(&surface_normal) * surface_normal));
        let r_parallel = -f64::sqrt(1. - r_orthogonal.magnitude_squared()) * surface_normal;
        Ray {
            origin: new_origin,
            direction_unit: r_orthogonal + r_parallel,
        }
    }

    #[inline]
    fn random_cosine_distribution() -> f64 {
        f64::asin(fastrand::f64() * 2. - 1.)
    }

    #[inline]
    fn rodrigues_rotation(vector: Vec3, axis_of_rotation: Vec3, angle_radians: f64) -> Vec3 {
        vector * angle_radians.cos()
            + axis_of_rotation.cross(&vector) * angle_radians.sin()
            + axis_of_rotation * (axis_of_rotation.dot(&vector)) * (1. - angle_radians.cos())
    }

    #[inline]
    fn rotate_towards_vector(vector: Vec3, target_vector: Vec3, angle_radians: f64) -> Vec3 {
        let axis_of_rotation = vector.cross(&target_vector);
        vector * angle_radians.cos()
            + target_vector * angle_radians.sin()
            + axis_of_rotation * (axis_of_rotation.dot(&vector)) * (1. - angle_radians.cos())
    }
}
