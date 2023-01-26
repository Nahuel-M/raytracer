
use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{MulAssign, AddAssign};

use crate::algebra::{quaternion::Quaternion};
use crate::hit::Hit;
#[allow(unused)]
use crate::iter_functions::AllEqual;
use crate::ray::Ray;
use crate::Vec3;

use super::Shape;

#[derive(Debug)]
pub struct Polygon {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
}
#[allow(dead_code)]
impl Polygon {
    pub fn new_triangle_looking_at_position(
        triangle_position: Vec3,
        looking_position: Vec3,
        height: f64,
    ) -> Self {
        let looking_vector = (looking_position - triangle_position).normalize();
        let rotation_quaternion = Quaternion::from_unit_vectors(Vec3::z(), looking_vector);
        let vertex_to_center = height / 1.5;
        let mut vertices = [
            Vec3::new(0., vertex_to_center, 0.),
            Vec3::new(
                vertex_to_center * (PI / 6.).cos(),
                -vertex_to_center * (PI / 6.).sin(),
                0.,
            ),
            Vec3::new(
                -vertex_to_center * (PI / 6.).cos(),
                -vertex_to_center * (PI / 6.).sin(),
                0.,
            ),
        ];
        vertices.iter_mut().for_each(|vertex| {
            rotation_quaternion.rotate_vector(vertex);
            *vertex += triangle_position;
        });
        let normal =
            -Vec3::cross(&(vertices[1] - vertices[0]), &(vertices[2] - vertices[0])).normalize();
        let parallel_to_surface = (vertices[0] - vertices[1]).normalize();
        Polygon {
            vertices,
            normal,
            parallel_to_surface,
        }
    }
    pub fn new(p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> Self {
        let points = [
            Vec3::new(p1.0, p1.1, p1.2),
            Vec3::new(p2.0, p2.1, p2.2),
            Vec3::new(p3.0, p3.1, p3.2),
        ];

        Polygon {
            vertices: points,
            normal: -Vec3::cross(&(points[1] - points[0]), &(points[2] - points[0])).normalize(),
            parallel_to_surface: (points[0] - points[1]).normalize(),
        }
    }

    pub fn with_normal(
        p1: (f64, f64, f64),
        p2: (f64, f64, f64),
        p3: (f64, f64, f64),
        normal: (f64, f64, f64),
    ) -> Self {
        let points = [
            Vec3::new(p1.0, p1.1, p1.2),
            Vec3::new(p2.0, p2.1, p2.2),
            Vec3::new(p3.0, p3.1, p3.2),
        ];

        Polygon {
            vertices: points,
            normal: Vec3::new(normal.0, normal.1, normal.2),
            parallel_to_surface: (points[0] - points[1]).normalize(),
        }
    }
    #[allow(dead_code)]
    pub fn calculate_normal(&self) -> Vec3 {
        let vec1 = self.vertices[1] - self.vertices[0];
        let vec2 = self.vertices[2] - self.vertices[0];
        vec1.cross(&vec2).normalize()
    }
    pub fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let distance_to_plane = self.normal.dot(&(self.vertices[0] - ray.origin))
            / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);
        let directions = [
            self.vertices[1] - self.vertices[0],
            self.vertices[2] - self.vertices[1],
            self.vertices[0] - self.vertices[2],
        ];
        if directions
            .iter()
            .zip(self.vertices.to_vec())
            .any(|(direction, vertex)| {
                (point_on_plane - vertex).cross(direction).dot(&self.normal) < 0.0
            })
            // .all_equal()
        {
            return None;
        }
        Some(distance_to_plane)
    }
    pub fn bounding_points(&self) -> [Vec3; 2]{
        [
            Vec3::new(self.vertices[0].x.min(self.vertices[1].x).min(self.vertices[2].x),
            self.vertices[0].y.min(self.vertices[1].y).min(self.vertices[2].y),
            self.vertices[0].z.min(self.vertices[1].z).min(self.vertices[2].z)),
            Vec3::new(self.vertices[0].x.max(self.vertices[1].x).max(self.vertices[2].x),
            self.vertices[0].y.max(self.vertices[1].y).max(self.vertices[2].y),
            self.vertices[0].z.max(self.vertices[1].z).max(self.vertices[2].z)),
        ]           
    }
}

impl Display for Polygon{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Polygon with normal: ".to_string() + self.normal.to_string().as_str())
    }
}

impl Shape for Polygon{
    fn get_potential_hit(&self, ray: &crate::ray::Ray) -> Option<Hit> {
        let potential_hit = self.get_hit_distance(ray);

        if let Some(distance) = potential_hit{
            return Some(Hit{
                distance,
                position: ray.at(distance),
                normal: self.normal,
                parallel_to_surface: self.parallel_to_surface,
            })
        }
        None
    }
}

impl MulAssign<f64> for Polygon{
    fn mul_assign(&mut self, rhs: f64) {
        self.vertices.iter_mut().for_each(|vertex| *vertex *= rhs);
    }
}

impl AddAssign<Vec3> for Polygon{
    fn add_assign(&mut self, rhs: Vec3) {
        self.vertices.iter_mut().for_each(|vertex| *vertex += rhs);
    }
}