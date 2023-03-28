use std::{fmt::{Display, Debug}, ops::AddAssign};

use crate::{algebra::vec3::Vec3};

use super::{triangle::Triangle};

pub type Vertex = Vec3;
pub type VertexNormal = Vec3;
pub type UV = (f64, f64);
#[derive(Default)]
pub struct Model{
    pub vertices : Vec<Vertex>,
    pub vertex_normals: Vec<VertexNormal>,
    pub vertex_uv: Vec<UV>,
    pub faces : Vec<Triangle>,
    pub material_name: String,
}

impl Debug for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Model with {} vertices, {} faces and material {}", self.vertices.len(), self.faces.len(), self.material_name)
    }
}

impl AddAssign<Vec3> for Model{
    fn add_assign(&mut self, rhs: Vec3) {
        for vertex in &mut self.vertices{
            vertex.add_assign(rhs);
        }
    }
}
