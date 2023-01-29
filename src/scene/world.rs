use std::fmt::Display;

use crate::{algebra::vec3::Vec3, hittable::Hittable};

use super::camera::Camera;

pub struct World<'a>{
    pub camera: Camera,
    pub hittables: Vec<Hittable<'a>>,
    pub background: Vec3,
}

impl World<'_>{
    pub fn with_camera(camera: Camera) -> Self{
        World{camera, hittables: vec![], background: Vec3::zeros() }
    }
}

impl Display for World<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.camera).unwrap();
        for object in &self.hittables{
            writeln!(f, "{}", object).unwrap();
        }
        writeln!(f)
    }
}