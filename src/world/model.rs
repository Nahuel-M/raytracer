use std::{fmt::Display, sync::{Arc, RwLock}, ops::AddAssign};

use crate::{material::Material, algebra::vec3::Vec3};

use super::vertex::Vertex;

pub struct Model{
    pub vertices : Vec<Vertex>,
    pub material : Arc<RwLock<Material>>
}

impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model with {} vertices.", self.vertices.len())
    }
}

impl AddAssign<Vec3> for Model{
    fn add_assign(&mut self, rhs: Vec3) {
        for vertex in &mut self.vertices{
            vertex.get_mut().add_assign(rhs);
        }
    }
}