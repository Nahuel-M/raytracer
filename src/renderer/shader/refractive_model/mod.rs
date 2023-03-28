use crate::{hit::Hit, algebra::{ray::Ray, vec3::Vec3}, renderer::tracer::trace_package::TracePackage};

use super::shade_package::ShadePackage;


#[allow(dead_code)]
pub enum RefractiveModel {
    None,
    SchlickFresnell,
}
impl RefractiveModel {
    #[inline]
    pub fn add_refraction(&self, hit : &Hit, ray: &Ray, package_vec: &mut Vec<ShadePackage>, refraction_factor: f64){
        package_vec.push(TracePackage {
            ray: ray.refract(hit.normal, hit.position, hit.material.ior),
            multiplier: Vec3::new(refraction_factor, refraction_factor, refraction_factor)
        }.into());
    }
}