use std::sync::{Arc, RwLock, RwLockWriteGuard};

use crate::algebra::vec3::Vec3;

#[derive(Debug)]
pub struct Vertex(Arc<RwLock<Vec3>>);

impl Vertex{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vertex(Arc::new(RwLock::new(Vec3::new(x, y, z))))
    }
    pub fn get(&self) -> Vec3{
        *self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, Vec3 >{
        self.0.write().unwrap()
    }
}

impl Clone for Vertex{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
