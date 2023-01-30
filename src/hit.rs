use crate::{algebra::vec3::Vec3, shape::{sphere::Sphere, triangle::Triangle}};
#[derive(Debug)]
pub struct Hit {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
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
        }
    }
}
#[derive(Debug)]
pub struct MeshHit<'a>{
    hit_triangle : &'a Triangle,
    hit_position : Vec3,
}

// #[derive(Debug)]
// pub struct PhongMeshHit<'a>(&'a  PhongTriangle);