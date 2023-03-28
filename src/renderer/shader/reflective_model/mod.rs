use std::f64::consts::PI;

use crate::{hit::Hit, algebra::{ray::Ray, vec3::Vec3, quaternion::Quaternion}, renderer::tracer::trace_package::TracePackage};

use super::shade_package::ShadePackage;
#[allow(dead_code)]
pub enum SpecularModel {
    None,
    CookTorrance(CookTorrance),
}
impl SpecularModel {
   
    #[inline]
    pub fn add_specular(&self, hit : &Hit, ray: &Ray, package_vec: &mut Vec<ShadePackage>, specular_factor: f64){
        match self{
            SpecularModel::None => {},
            SpecularModel::CookTorrance(cook_torrance) => cook_torrance.add_specular(hit, ray, package_vec, specular_factor),
        }
    }
}

pub struct CookTorrance {
    pub(crate) distribution_function: SpecularDistributionFunction,
    pub(crate) geometry_function: SpecularGeometryFunction,
}

impl CookTorrance{
    #[inline]
    pub fn add_specular(&self, hit : &Hit, ray: &Ray, package_vec: &mut Vec<ShadePackage>, specular_factor: f64){
        let normal = self.distribution_function.micro_facet_normal_sample(hit.material.roughness, &hit.normal);
        package_vec.push(TracePackage {
            ray: ray.reflect_specular(normal, hit.position),
            multiplier: Vec3::uniform(specular_factor) * self.geometry_function.get_shading_factor(&hit.material.roughness, ray, &hit.normal),
        }.into());
    }
}
pub enum SpecularGeometryFunction {
    // See https://learnopengl.com/PBR/Theory
    SchlickGGX,
}

impl SpecularGeometryFunction{
    pub fn get_shading_factor(&self, roughness: &f64, ray: &Ray, surface_normal: &Vec3) -> f64{
        let alpha = roughness * roughness / 2.;
        let dot = -ray.direction_unit.dot(surface_normal);

        dot / (dot * (1.-alpha) + alpha)
    }
}

#[allow(dead_code)]
pub enum SpecularDistributionFunction {
    Ggx,
    Phong,
}
// See https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf
impl SpecularDistributionFunction{
    #[inline]
    fn micro_facet_normal_sample(&self, roughness: f64, surface_normal : &Vec3) -> Vec3{
        match self{
            SpecularDistributionFunction::Ggx => {
                let random_u = fastrand::f64();
                let random_v = fastrand::f64();

                let parallel = 1. / ((roughness*roughness * random_u / (1.-random_u)) + 1. ).sqrt();
                let flat_radius = (1. - parallel * parallel).sqrt();
                let flat_angle = 2. * PI * random_v;

                let mut random_cos_hemisphere = Vec3::new(
                    flat_radius * flat_angle.cos(),
                    flat_radius * flat_angle.sin(),
                    parallel
                );

                let align_with_normal = Quaternion::from_unit_vectors(&Vec3::Z, surface_normal);

                align_with_normal.rotate_vector(&mut random_cos_hemisphere);

                random_cos_hemisphere
            
            },
            SpecularDistributionFunction::Phong => todo!(),
        }
    }
}

#[allow(dead_code)]
pub enum DiffuseModel {
    None,
    Lambertian(usize),
}

impl DiffuseModel{
    // pub fn count(&self) -> usize{
    //     match self {
    //         DiffuseModel::None => 0,
    //         DiffuseModel::Lambertian(count) => *count,
    //     }
    // }
    
    #[inline]
    pub fn add_diffuse(&self, hit : &Hit, ray: &Ray, package_vec: &mut Vec<ShadePackage>, diffuse_factor: Vec3){
        match self{
            DiffuseModel::None => {},
            DiffuseModel::Lambertian(count) => {
                for _ in 0..*count{
                    let diffuse_factor = diffuse_factor / *count as f64;
                    package_vec.push(TracePackage {
                        ray: ray.reflect_diffuse(hit.normal, hit.position),
                        multiplier: diffuse_factor,
                    }.into());
                }
            },
        }
        
    }
}
