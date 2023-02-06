use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{AddAssign, MulAssign};

use crate::algebra::quaternion::Quaternion;
use crate::hit::Hit;

use crate::Vec3;
use crate::ray::Ray;

use super::Shape;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
    edge_1: Vec3,
    edge_2: Vec3,
    v_1: Vec3,
    v_2: Vec3,
}
#[allow(dead_code)]
impl Triangle {
    pub fn looking_at_position(
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
        Triangle {
            vertices,
            normal,
            parallel_to_surface,
            edge_1: Vec3::zeros(),
            edge_2: Vec3::zeros(),
            v_1: Vec3::zeros(),
            v_2: Vec3::zeros(),
        }
    }
    pub fn new(p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> Self {
        let points = [
            Vec3::new(p1.0, p1.1, p1.2),
            Vec3::new(p2.0, p2.1, p2.2),
            Vec3::new(p3.0, p3.1, p3.2),
        ];

        Triangle {
            vertices: points,
            normal: -Vec3::cross(&(points[1] - points[0]), &(points[2] - points[0])).normalize(),
            parallel_to_surface: (points[0] - points[1]).normalize(),
            edge_1: Vec3::zeros(),
            edge_2: Vec3::zeros(),
            v_1: Vec3::zeros(),
            v_2: Vec3::zeros(),
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

        Triangle {
            vertices: points,
            normal: Vec3::new(normal.0, normal.1, normal.2),
            parallel_to_surface: (points[0] - points[1]).normalize(),
            edge_1: Vec3::zeros(),
            edge_2: Vec3::zeros(),
            v_1: Vec3::zeros(),
            v_2: Vec3::zeros(),
        }
    }

    pub fn get_distance(&self, ray: &Ray) -> Option<(f64, Vec3)> {
        let distance_to_plane = self.normal.dot(&(self.vertices[0] - ray.origin))
            / self.normal.dot(&ray.direction_unit);
        let point_on_plane = ray.at(distance_to_plane);

        let a =
            1. - (self.vertices[1] - point_on_plane).dot(&self.v_1) / self.edge_1.dot(&self.v_1);
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let b =
            1. - (self.vertices[2] - point_on_plane).dot(&self.v_2) / self.edge_2.dot(&self.v_2);
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some((distance_to_plane, point_on_plane))
    }

    #[allow(dead_code)]
    pub fn calculate_normal(&self) -> Vec3 {
        let vec1 = self.vertices[1] - self.vertices[0];
        let vec2 = self.vertices[2] - self.vertices[0];
        vec1.cross(&vec2).normalize()
    }

    #[allow(clippy::manual_range_contains)]

    pub fn bounding_points(&self) -> [Vec3; 2] {
        [
            Vec3::new(
                self.vertices[0]
                    .x
                    .min(self.vertices[1].x)
                    .min(self.vertices[2].x),
                self.vertices[0]
                    .y
                    .min(self.vertices[1].y)
                    .min(self.vertices[2].y),
                self.vertices[0]
                    .z
                    .min(self.vertices[1].z)
                    .min(self.vertices[2].z),
            ),
            Vec3::new(
                self.vertices[0]
                    .x
                    .max(self.vertices[1].x)
                    .max(self.vertices[2].x),
                self.vertices[0]
                    .y
                    .max(self.vertices[1].y)
                    .max(self.vertices[2].y),
                self.vertices[0]
                    .z
                    .max(self.vertices[1].z)
                    .max(self.vertices[2].z),
            ),
        ]
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            "Polygon with normal: ".to_string() + self.normal.to_string().as_str()
        )
    }
}

impl Shape for Triangle {
    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        let potential_hit = self.get_distance(ray);

        if let Some((distance, position)) = potential_hit {
            return Some(Hit {
                distance,
                position,
                normal: self.normal,
                parallel_to_surface: self.parallel_to_surface,
            })
        }
        None
    }

    fn pre_compute(&mut self) {
        self.edge_1 = self.vertices[1] - self.vertices[0];
        self.edge_2 = self.vertices[2] - self.vertices[0];

        self.v_1 = self.edge_1 - self.edge_1.project(&self.edge_2);
        self.v_2 = self.edge_2 - self.edge_2.project(&self.edge_1);
    }
}

impl MulAssign<f64> for Triangle {
    fn mul_assign(&mut self, rhs: f64) {
        self.vertices.iter_mut().for_each(|vertex| *vertex *= rhs);
    }
}

impl AddAssign<Vec3> for Triangle {
    fn add_assign(&mut self, rhs: Vec3) {
        self.vertices.iter_mut().for_each(|vertex| *vertex += rhs);
    }
}
