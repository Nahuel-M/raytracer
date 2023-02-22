pub mod shade_package;

use std::f64::consts::PI;

use crate::{
    algebra::{vec3::Vec3, ray::Ray},
    hit::{Hit, TraceResult},
    material::map::{GetValueAt, RgbMap},
    renderer::tracer::trace_package::TracePackage,
    world::World,
};

use self::shade_package::ShadePackage;

pub struct Shader<'a> {
    reflective_model: ReflectiveModel,
    refractive_model: RefractiveModel,

    scene_background: Option<&'a RgbMap>,
}

impl<'a> Shader<'a> {
    pub fn pre_compute(&mut self, world: &'a World) {
        self.scene_background = Some(&world.background);
    }

    pub fn shade_hit(&self, trace_result: &TraceResult, ray : &Ray) -> Vec<ShadePackage> {
        match trace_result {
            TraceResult::Hit(hit) => Shader::parse_hit(hit, ray).into(),
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

    fn parse_hit(hit: &Hit, ray : &Ray) -> Vec<ShadePackage> {
        let mut packages : Vec<ShadePackage> = vec![];
        let material = hit.material.unwrap();

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
                packages.push(TracePackage {
                    ray: ray.reflect_specular(hit.normal, hit.position),
                    multiplier: Vec3::new(specular_factor, specular_factor, specular_factor),
                }.into())
            }

            // Add diffuse
            if diffuse_factor > 0.0001 {
                packages.push(TracePackage {
                    ray: ray.reflect_diffuse(hit.normal, hit.position),
                    multiplier: diffuse_factor * material.diffuse_color
                }.into());
            }
        } else {
            // Continue ray as if nothing happened
            packages.push(TracePackage {
                ray: Ray{origin: hit.position, ..*ray},
                multiplier: Vec3::ONES
            }.into());
        }

        // Add refraction
        if refraction_factor > 0.01 {
            packages.push(TracePackage {
                ray: ray.refract(hit.normal, hit.position, material.ior),
                multiplier: Vec3::new(refraction_factor, refraction_factor, refraction_factor)
            }.into());
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

pub enum RefractiveModel {
    None,
    SchlickFresnell,
}

pub struct ReflectiveModel {
    diffuse_model: DiffuseModel,
    specular_model: SpecularModel,
}

pub enum SpecularModel {
    None,
    CookTorrance(CookTorrance),
}

pub struct CookTorrance {
    distribution_function: SpecularDistributionFunction,
    geometry_function: SpecularGeometryFunction,
}

pub enum SpecularGeometryFunction {
    GGX,
}

pub enum SpecularDistributionFunction {
    GGX,
    Phong,
}

pub enum DiffuseModel {
    None,
    Lambertian,
}

impl Default for Shader<'_> {
    fn default() -> Self {
        Self {
            reflective_model: ReflectiveModel {
                diffuse_model: DiffuseModel::None,
                specular_model: SpecularModel::None,
            },
            refractive_model: RefractiveModel::None,
            scene_background: Default::default(),
        }
    }
}
