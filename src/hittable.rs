use core::fmt;
use std::fmt::Display;

use crate::{
    material::Material,
    ray::Ray,
    shape::{triangle::Triangle, sphere::Sphere, Shape, model::Model},
    Vec3, hit::Hit,
};
pub struct Hittable<'a> {
    pub shape: Box<dyn Shape + 'a>,
    pub material: Material,
}

impl Display for Hittable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.shape)
    }
}
impl fmt::Debug for Hittable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<Model> for Hittable<'_>{
    fn from(value: Model) -> Self {
        Hittable{ shape: Box::new(value), material: Material::base_diffuse() }
    }
}

#[allow(dead_code)]
impl Hittable<'_> {
    pub fn sphere() -> Self {
        Hittable {
            shape: Box::new(Sphere::new(0, 0, 0, 1)),
            material: Material::new(Vec3::new(1., 1., 1.)),
        }
    }
    pub fn with_sphere(sphere: Sphere) -> Self {
        Hittable {
            shape: Box::new(sphere),
            material: Material::new(Vec3::new(1., 1., 1.)),
        }
    }
    pub fn with_polygon(polygon: Triangle) -> Self {
        Hittable {
            shape: Box::new(polygon),
            material: Material::new(Vec3::new(1., 1., 1.)),
        }
    }
    pub fn with_color(mut self, r: f64, g: f64, b: f64) -> Self {
        self.material.color = Vec3::new(r, g, b);
        self
    }
    pub fn with_luminance(mut self, r: f64, g: f64, b: f64) -> Self {
        self.material.luminance = Vec3::new(r, g, b);
        self
    }
    pub fn with_specular<T: Into<f64>>(mut self, specular: T) -> Self {
        self.material.specular = specular.into();
        self
    }
    pub fn with_refraction<T: Into<f64>>(mut self, refraction: T) -> Self {
        self.material.refraction = refraction.into();
        self
    }
    pub fn with_ior<T: Into<f64>>(mut self, ior: T) -> Self {
        self.material.ior = ior.into();
        self
    }
    pub fn get_potential_hit(&self, ray : &Ray) -> Option<Hit>{
        self.shape.get_potential_hit(ray)
    }
}
