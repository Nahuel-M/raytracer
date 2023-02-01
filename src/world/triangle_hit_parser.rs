use crate::{algebra::vec3::Vec3, ray::Ray};


pub struct TriangleHitParser{
    normal: Vec3,

    p1 : Vec3, // Two vertices of the triangle
    p2 : Vec3,

    edge_1 : Vec3, // Two vectors in the direction of edges of the triangle
    edge_2 : Vec3,

    v1 : Vec3, // Rejection of other edge
    v2 : Vec3,
} // 7*3*8= 168 bytes

impl TriangleHitParser{
    pub fn get_hit_distance(&self, ray: &Ray) -> Option<f64>{

        let distance_to_plane = 
            self.normal.dot(&(self.p1 - ray.origin))
            / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);

        let a = 1. - (self.p1 - point_on_plane).dot(&self.v1) / self.edge_1.dot(&self.v1);
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let b = 1. - (self.p2 - point_on_plane).dot(&self.v2) / self.edge_2.dot(&self.v2);
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }
}