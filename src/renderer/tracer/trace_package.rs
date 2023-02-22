use crate::algebra::{ray::Ray, vec3::Vec3};

pub struct TracePackage{
    pub ray: Ray,
    pub multiplier : Vec3,
}