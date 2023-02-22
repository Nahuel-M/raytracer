use crate::Vec3;

use super::quaternion::Quaternion;

#[derive(Debug, Clone, Copy)]
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
        new_origin: Vec3,
    ) -> Self {
        let random_u = fastrand::f64();
        let random_v = fastrand::f64();

        let radius = random_u.sqrt();
        let theta = 2.0 * std::f64::consts::PI * random_v;

        let mut random_cos_hemisphere = Vec3::new(
            radius * theta.cos(),
            radius * theta.sin(),
            (1. - random_u).sqrt()
        );

        let align_with_normal = Quaternion::from_unit_vectors(Vec3::Z, surface_normal);

        align_with_normal.rotate_vector(&mut random_cos_hemisphere);

        Ray {
            origin: new_origin,
            direction_unit: random_cos_hemisphere,
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
}
