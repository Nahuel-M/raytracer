use core::fmt;
use std::fmt::Display;

use crate::{
    material::Material,
    shape::{triangle::Triangle, sphere::Sphere, Shape, mesh::Mesh},
    Vec3, hit::Hit, ray::Ray,
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
impl<'a> From<Mesh> for Hittable<'a>{
    fn from(value: Mesh) -> Self {
        Hittable{ shape: Box::new(value), material: Material::base_diffuse() }
    }
}

#[allow(dead_code)]
impl<'a> Hittable<'a> {
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

    // pub fn propagate_ray(&self, ray: &Ray, hit: Hit, world : &World, remaining_depth : u8) -> Vec3{
    //     let mut final_color = Vec3::new(0., 0., 0.);
        
    //     final_color += self.material.luminance;
    //     if self.material.specular > 0.001 {
    //         final_color += self.material.specular * {
    //             let specular_ray = ray.reflect_specular(hit.normal, hit.position);
    //             Scene::color_for_ray(&specular_ray, world, remaining_depth - 1)
    //         };
    //     }
    //     if self.material.specular + self.material.refraction < 0.999 && self.material.color.sum() > 0.001 {
    //         final_color += (1. - self.material.specular - self.material.refraction) * self.material.color * {
    //             let diffuse_ray = ray.reflect_diffuse(hit.normal, hit.parallel_to_surface, hit.position);
    //             Scene::color_for_ray(&diffuse_ray, world, remaining_depth - 1)
    //         };
    //     }
    //     if self.material.refraction > 0.001 {
    //         final_color += self.material.refraction * {
    //             let refract_ray = ray.refract(hit.normal, hit.position, self.material.ior);
    //             Scene::color_for_ray(&refract_ray, world, remaining_depth - 1)
    //         };
    //     }
    //     final_color
    // }

    pub(crate) fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        self.shape.get_hit(ray)
    }

}
