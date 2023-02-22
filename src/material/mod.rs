pub mod map;

use crate::Vec3;

// use self::map::RgbMap;

#[derive(Debug)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub luminance: Vec3,
    pub refraction: f64,
    pub ior: f64,
    pub specular_color: Vec3,
    pub specular: f64,
    pub roughness: f64,
}

#[allow(dead_code)]
impl Material {
    pub fn new(color: Vec3) -> Self {
        Material {
            diffuse_color: color,
            specular_color: Vec3::ONES,
            luminance: Vec3::new(0., 0., 0.),
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }

    pub fn base_diffuse() -> Self {
        Material {
            diffuse_color: Vec3::new(0.5, 0.5, 0.5),
            specular_color: Vec3::ONES,
            luminance: Vec3::new(0., 0., 0.),
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }
    pub fn as_light(luminance: Vec3) -> Self {
        Material {
            diffuse_color: Vec3::new(0., 0., 0.),
            specular_color: Vec3::ZEROS,
            luminance,
            refraction: 0.,
            ior: 1.,
            specular: 0.,
            roughness: 0.,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::base_diffuse()
    }
}
