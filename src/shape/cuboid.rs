use std::{fmt::Display, ops::{Add, AddAssign, MulAssign}};

use crate::{algebra::vec3::Vec3, ray::Ray, hit::Hit};

use super::Shape;

#[derive(Debug)]
pub struct Cuboid{
    pub min_vertex : Vec3,
    pub max_vertex : Vec3,
}

impl Display for Cuboid{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cuboid between {} and {}", self.min_vertex, self.max_vertex)
    }
}

impl Cuboid{
    pub fn test_collision(&self, ray : &Ray) -> bool{
        let min_vertex = self.min_vertex - ray.origin;
        let max_vertex = self.max_vertex - ray.origin;

        let distances_to_min_vertex = min_vertex / ray.direction_unit;
        let distances_to_max_vertex = max_vertex / ray.direction_unit;

        let min_distance = distances_to_min_vertex.ew_min(distances_to_max_vertex);
        let max_distance = distances_to_min_vertex.ew_max(distances_to_max_vertex);

        min_distance.max() < max_distance.min()

    }
}

impl AddAssign<Vec3> for Cuboid{
    fn add_assign(&mut self, rhs: Vec3) {
        self.min_vertex += rhs;
        self.max_vertex += rhs;
    }
}

impl MulAssign<f64> for Cuboid{
    fn mul_assign(&mut self, rhs: f64) {
        self.max_vertex *= rhs;
        self.min_vertex *= rhs;
    }
}

// impl Shape for Cuboid{
//     fn get_potential_hit(&self, ray: &Ray) -> Option<Hit> {
//         todo!()
//     }
// }
