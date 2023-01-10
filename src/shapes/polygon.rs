use nalgebra::Vector3;
use crate::{renderer::RayRenderable, ray::Ray, iter_functions::AllEqual};

#[derive(Debug)]
pub struct Polygon {
    pub vertices: [Vector3<f64>; 3],
    pub normal: Vector3<f64>,
}

impl Polygon {
    pub fn new(p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> Self {
        let points = [
            Vector3::new(p1.0, p1.1, p1.2),
            Vector3::new(p2.0, p2.1, p2.2),
            Vector3::new(p3.0, p3.1, p3.2),
        ];
        Polygon {
            vertices: points,
            normal: Vector3::cross(&(points[1] - points[0]), &(points[2] - points[0])).normalize(),
        }
    }
    pub fn calculate_normal(&self) -> Vector3<f64>{
        let vec1 = self.vertices[1] - self.vertices[0];
        let vec2 = self.vertices[2] - self.vertices[0];
        vec1.cross(&vec2).normalize()
    }
}

impl RayRenderable for Polygon {
    fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let distance_to_plane =
            self.normal.dot(&(self.vertices[0] - ray.origin)) / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);
        let directions = [
            self.vertices[1] - self.vertices[0],
            self.vertices[2] - self.vertices[1],
            self.vertices[0] - self.vertices[2],
        ];
        if directions
            .iter()
            .zip(self.vertices.to_vec())
            .any(|(direction, vertex)| (point_on_plane-vertex).cross(direction).dot(&self.normal) < 0.0)
        {
            return None;
        }
        Some(distance_to_plane)
    }
}
