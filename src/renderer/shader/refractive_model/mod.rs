use crate::{hit::Hit, algebra::{ray::Ray, vec3::Vec3}, renderer::tracer::trace_package::TracePackage};

use super::shade_package::ShadePackage;


pub enum RefractiveModel {
    None,
    SchlickFresnell,
}
impl RefractiveModel {
    pub(crate) fn count(&self) -> usize {
        1
    }
    #[inline]
    pub fn add_refraction(&self, hit : &Hit, ray: &Ray, package_vec: &mut Vec<ShadePackage>, refraction_factor: f64){
        package_vec.push(TracePackage {
            ray: ray.refract(hit.normal, hit.position, hit.material.ior),
            multiplier: Vec3::new(refraction_factor, refraction_factor, refraction_factor)
        }.into());
    }
}