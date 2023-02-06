use std::sync::{Arc, RwLock};

use crate::{algebra::vec3::Vec3, material::Material};

use super::{
    triangle_hit_parser::TriangleHitParser, Vertex,
};

// #[derive(Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub normal: Vec3,
    pub material: Arc<RwLock<Material>>,
    pub vertex_normal_indexes : Option<[usize; 3]>,
    pub vertex_color_indexes : Option<[usize; 3]>,
}

impl Triangle {
    pub fn generate_hit_parser(&self) -> TriangleHitParser {
        let p0 = self.vertices[0].get();
        let p1 = self.vertices[1].get();
        let p2 = self.vertices[2].get();

        let edge_1 = p1 -p0;
        let edge_2 = p2 -p0;

        let v1 = edge_1 - edge_1.project(&edge_2);
        let v2 = edge_2 - edge_2.project(&edge_1);

        let inv_proj_1 = 1./edge_1.dot(&v1);
        let inv_proj_2 = 1./edge_2.dot(&v2);

        TriangleHitParser {
            normal: self.normal,
            vertices: [p0, p1, p2],
            inv_proj_1,
            inv_proj_2,
            v1,
            v2,
        }
    }
}
