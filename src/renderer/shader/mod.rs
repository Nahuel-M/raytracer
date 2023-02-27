pub mod shade_package;
mod reflective_model;
mod refractive_model;

use std::f64::consts::PI;

use crate::{
    algebra::{vec3::Vec3, ray::Ray},
    hit::{Hit, TraceResult},
    material::map::{GetValueAt, RgbMap},
    renderer::tracer::trace_package::TracePackage,
    world::World,
};

use self::{shade_package::ShadePackage, reflective_model::{DiffuseModel, SpecularModel, CookTorrance, SpecularDistributionFunction}, refractive_model::RefractiveModel};

pub struct Shader<'a> {
    diffuse_model: DiffuseModel,
    specular_model: SpecularModel,
    refractive_model: RefractiveModel,
    // ray_count : usize,

    scene_background: Option<&'a RgbMap>,
}

impl<'a> Shader<'a> {
    pub fn pre_compute(&mut self, world: &'a World) {
        self.scene_background = Some(&world.background);
        // self.ray_count = self.reflective_model.count() + self.refractive_model.count();
    }
    pub fn shade_hit(&self, trace_result: &TraceResult, ray : &Ray) -> Vec<ShadePackage> {
        match trace_result {
            TraceResult::Hit(hit) => self.parse_hit(hit, ray),
            TraceResult::Miss => match self.scene_background.unwrap() {
                RgbMap::Color(color) => vec![(*color).into()],
                RgbMap::Texture(texture) => {
                    let u = 0.5 + f64::atan2(ray.direction_unit.z, ray.direction_unit.x) / (2. * PI);
                    let v = 0.5 - f64::asin(ray.direction_unit.y.min(1.).max(-1.)) / PI;
                    vec![texture.get_value_at(u, v).into()]
                }
            }
        }
    }
    fn parse_hit(&self, hit: &Hit, ray : &Ray) -> Vec<ShadePackage> {
        let mut packages : Vec<ShadePackage> = vec![];
        let material = hit.material;

        let base_reflectance = (material.ior - 1.).powi(2) / (material.ior + 1.).powi(2);
        let fresnel_reflection = Shader::schlick_fresnell_approximation(
            base_reflectance,
            &hit.normal,
            ray.direction_unit,
        );

        let specular_factor = (1. - material.refraction) * material.specular
            + material.refraction * fresnel_reflection;
        let refraction_factor = material.refraction * (1. - fresnel_reflection);
        let diffuse_factor = (1. - material.specular) * (1. - material.refraction);

        let hitting_face_from_front = ray.direction_unit.dot(&hit.normal) < 0.;
        if hitting_face_from_front {
            // Add luminance
            if material.luminance.sum() > 0.{
                packages.push(material.luminance.into());
            }

            // Add specular
            if specular_factor > 0.0001 {
                self.specular_model.add_specular(hit, ray, &mut packages, specular_factor);
            }

            // Add diffuse
            if diffuse_factor > 0.0001 {
                self.diffuse_model.add_diffuse(hit, ray, &mut packages, diffuse_factor * material.diffuse_color);
            }
        } else {
            // Continue ray as if nothing happened
            packages.push(TracePackage {
                ray: Ray{origin: hit.position, ..*ray},
                multiplier: Vec3::ONES
            }.into());
        }

        // Add refraction
        if refraction_factor > 0.0001 {
            self.refractive_model.add_refraction(hit, ray, &mut packages, refraction_factor);
        }

        packages
    }


    #[inline]
    /// See https://en.wikipedia.org/wiki/Schlick%27s_approximation
    fn schlick_fresnell_approximation(
        base_reflectance: f64,
        surface_normal: &Vec3,
        ray_direction: Vec3,
    ) -> f64 {
        base_reflectance
            + (1. - base_reflectance) * (1. - ray_direction.dot(&-*surface_normal)).powi(5)
    }

    pub(crate) fn clear(&mut self) {
        self.scene_background = None;
    }
}



impl Default for Shader<'_> {
    fn default() -> Self {
        Self {
            diffuse_model: DiffuseModel::Lambertian(2),
            specular_model: SpecularModel::CookTorrance(
                CookTorrance{ distribution_function: SpecularDistributionFunction::GGX, 
                    geometry_function: reflective_model::SpecularGeometryFunction::GGX }),
            refractive_model: RefractiveModel::None,
            scene_background: Default::default(),
        }
    }
}
