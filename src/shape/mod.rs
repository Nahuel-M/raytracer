use std::fmt::Display;

use crate::hit::Hit;

pub mod sphere;
pub mod polygon;
pub mod model;
pub mod cuboid;
#[allow(dead_code)]

// pub enum Shape{
//     Sphere(Sphere),
//     Polygon(Polygon),
//     Model(Model),
// }

pub trait Shape : Sync + Display{
    fn get_potential_hit(&self, ray: &crate::ray::Ray) -> Option<Hit>;
}