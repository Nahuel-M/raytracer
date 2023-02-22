use crate::{algebra::vec3::Vec3, material::Material};

#[derive(Debug)]
pub struct Hit <'a>{
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Option<&'a Material>,
}

pub enum TraceResult<'a>{
    Hit(Hit<'a>),
    Miss,
}