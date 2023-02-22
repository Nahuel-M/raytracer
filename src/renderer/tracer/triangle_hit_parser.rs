use crate::{algebra::{vec3::Vec3, axis::Axis}, algebra::ray::Ray};

pub struct TriangleHitParser {
    pub normal: Vec3,

    pub vertices: [Vec3; 3],

    pub inv_proj_1: f64, // Inv projection of edges in barycentric vector
    pub inv_proj_2: f64,

    pub v1: Vec3, // Barycentric vector
    pub v2: Vec3,

}

impl TriangleHitParser{
    pub fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let predot = self.normal.dot(&ray.direction_unit);
        if (-0.01..0.01).contains(&predot){
            return None
        }
        let distance_to_plane =
            self.normal.dot(&(self.vertices[1] - ray.origin)) / predot;

        if distance_to_plane < 0.01 {
            return None;
        }

        let point_on_plane = ray.at(distance_to_plane);

        let a = 1. - (self.vertices[1] - point_on_plane).dot(&self.v1) * self.inv_proj_1;
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let b = 1. - (self.vertices[2] - point_on_plane).dot(&self.v2) * self.inv_proj_2;
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }

    pub fn get_barycentric_a_b(&self, ray: &Ray) -> (f64, f64){
        let distance_to_plane =
            self.normal.dot(&(self.vertices[1] - ray.origin)) / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);

        let a = 1. - (self.vertices[1] - point_on_plane).dot(&self.v1) * self.inv_proj_1;
        let b = 1. - (self.vertices[2] - point_on_plane).dot(&self.v2) * self.inv_proj_2;
        (a, b)
    }

    pub fn get_center(&self) -> Vec3{
        self.vertices.iter().sum::<Vec3>() / 3.
    }
    #[allow(unused)]
    pub fn centroid(&self, axis: Axis) -> f64{
        match axis{
            Axis::X => self.vertices.iter().map(|vertex| vertex.x).sum::<f64>() / 3.0,
            Axis::Y => self.vertices.iter().map(|vertex| vertex.y).sum::<f64>() / 3.0,
            Axis::Z => self.vertices.iter().map(|vertex| vertex.z).sum::<f64>() / 3.0,
        }
    }

}
