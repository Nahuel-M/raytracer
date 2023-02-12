use std::sync::{Arc, RwLock};

use crate::{algebra::vec3::Vec3, material::Material};

#[derive(Debug)]
pub struct Hit {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Option<Arc<RwLock<Material>>>,
}