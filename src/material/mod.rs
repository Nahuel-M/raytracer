pub mod map;

use crate::Vec3;

#[derive(Debug)]
pub struct Material {
    pub diffuse: Vec3,
    pub luminance: Vec3,
    pub refraction: f64,
    pub ior: f64,
    pub specular: f64,
    pub roughness: f64,
}

#[allow(dead_code)]
impl Material {
    pub fn new(color: Vec3) -> Self {
        Material {
            diffuse: color,
            luminance: Vec3::new(0., 0., 0.),
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }

    pub fn base_diffuse() -> Self {
        Material {
            diffuse: Vec3::new(0.5, 0.5, 0.5),
            luminance: Vec3::new(0., 0., 0.),
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }
    pub fn as_light(luminance: Vec3) -> Self {
        Material {
            diffuse: Vec3::new(0., 0., 0.),
            luminance,
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }
}
