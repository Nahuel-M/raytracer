use crate::{algebra::vec3::Vec3, ray::Ray, material::Material};

use super::{
    triangle_hit_parser::TriangleHitParser, Vertex, VertexNormal, VertexColor,
};

#[derive(Clone, Copy)]
pub struct Triangle<'a> {
    pub vertex_indexes: [usize; 3],
    pub normal: Vec3,
    pub material: &'a Material,
    pub vertex_normals : Option<[&'a VertexNormal; 3]>,
    pub vertex_colors : Option<[&'a VertexColor; 3]>,
}

impl Triangle<'_> {
    pub fn get_hit_distance(&self, vertices: &Vec<Vertex>, ray: &Ray) -> Option<f64> {
        let i0 = self.vertex_indexes[0];
        let i1 = self.vertex_indexes[1];
        let i2 = self.vertex_indexes[2];

        let distance_to_plane =
            self.normal.dot(&(vertices[i0].0 - ray.origin)) / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);

        let edge_1 = vertices[i1].0 - vertices[i0].0;
        let edge_2 = vertices[i2].0 - vertices[i0].0;
        let v_1 = edge_1 - edge_1.project(&edge_2);

        let a = 1. - (vertices[i1].0 - point_on_plane).dot(&v_1) / edge_1.dot(&v_1);
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let v_2 = edge_2 - edge_2.project(&edge_1);

        let b = 1. - (vertices[i2].0 - point_on_plane).dot(&v_2) / edge_2.dot(&v_2);
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }

    pub fn generate_hit_parser(&self, vertices: &Vec<Vertex>) -> TriangleHitParser {
        let i0 = self.vertex_indexes[0];
        let i1 = self.vertex_indexes[1];
        let i2 = self.vertex_indexes[2];

        let edge_1 = vertices[i1].0 - vertices[i0].0;
        let edge_2 = vertices[i2].0 - vertices[i0].0;

        let v1 = edge_1 - edge_1.project(&edge_2);
        let v2 = edge_2 - edge_2.project(&edge_1);

        TriangleHitParser {
            normal: self.normal,
            p1: vertices[i1].0,
            p2: vertices[i2].0,
            edge_1,
            edge_2,
            v1,
            v2,
        }
    }
}
