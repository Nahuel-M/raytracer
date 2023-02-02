use std::sync::{Arc, RwLock};

use crate::{algebra::vec3::Vec3, material::Material};
static base_mat : Material = Material{ color: Vec3::zeros(), luminance: Vec3::zeros(), refraction: 0., ior: 0., 
    specular: 0., specular_roughness: 0. };
#[derive(Debug)]
pub struct Hit {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
    pub material: Option<Arc<RwLock<Material>>>,
}

// pub enum Hit<'a>{
//     Sphere(Sphere),
//     Triangle(Triangle),
//     Mesh(MeshHit<'a>),
//     PhongMesh(PhongMeshHit),

// }

impl Hit {
    pub fn max() -> Self {
        Hit {
            distance: f64::MAX,
            position: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            parallel_to_surface: Vec3::new(0., 0., 0.),
            material: None,
        }
    }
}
