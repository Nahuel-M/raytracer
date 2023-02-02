use crate::{algebra::vec3::Vec3, ray::Ray};

pub struct TriangleHitParser {
    pub normal: Vec3,

    pub p1: Vec3, // Two vertices of the triangle
    pub p2: Vec3,

    pub inv_proj_1: f64, // Inv projection of edges in barycentric vector
    pub inv_proj_2: f64,

    pub v1: Vec3, // Barycentric vector
    pub v2: Vec3,
} // 7*3*8= 168 bytes

impl TriangleHitParser {
    pub fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let distance_to_plane =
            self.normal.dot(&(self.p1 - ray.origin)) / self.normal.dot(&ray.direction_unit);

        if distance_to_plane < 0.0001 {
            return None;
        }

        let point_on_plane = ray.at(distance_to_plane);

        let a = 1. - (self.p1 - point_on_plane).dot(&self.v1) * self.inv_proj_1;
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let b = 1. - (self.p2 - point_on_plane).dot(&self.v2) * self.inv_proj_2;
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }

    pub fn get_barycentric_a_b(&self, ray: &Ray) -> (f64, f64){
        let distance_to_plane =
            self.normal.dot(&(self.p1 - ray.origin)) / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);

        let a = 1. - (self.p1 - point_on_plane).dot(&self.v1) * self.inv_proj_1;
        let b = 1. - (self.p2 - point_on_plane).dot(&self.v2) * self.inv_proj_2;
        (a, b)
    }
}
