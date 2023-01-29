use std::fmt::Display;

use crate::{hit::Hit, ray::Ray};

pub mod sphere;
pub mod triangle;
pub mod mesh;
pub mod cuboid;
// pub mod phong_mesh;


pub trait Shape : Sync + Display{
    fn pre_compute(&mut self);
    fn get_hit(&self, ray: &Ray) -> Option<Hit>;
}