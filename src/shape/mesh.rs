use std::{ops::{AddAssign, MulAssign}, fmt::Display};

use crate::{algebra::vec3::Vec3, hit::Hit, ray::Ray};

use super::{triangle::Triangle, cuboid::Cuboid, Shape};
#[derive(Debug)]
pub struct Mesh{
    pub polygons: Vec<Triangle>,
    pub bounding_box: Cuboid,
}

impl Mesh {
    pub fn new(polygons: Vec<Triangle>) -> Self {
        let bounding_points = polygons
            .iter()
            .map(|polygon| polygon.bounding_points())
            .reduce(|acc, new| {
                [
                    Vec3::new(acc[0].x.min(new[0].x), acc[0].y.min(new[0].y), acc[0].z.min(new[0].z), ),
                    Vec3::new(acc[1].x.max(new[1].x), acc[1].y.max(new[1].y), acc[1].z.max(new[1].z), ),
                ]
            })
            .unwrap();
        Mesh { polygons, bounding_box: Cuboid{min_vertex: bounding_points[0], max_vertex: bounding_points[1]}}
    }
}

impl Display for Mesh{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Model with {} polygons, with bounds: {}", self.polygons.len(), self.bounding_box)
    }
}

impl Shape for Mesh{
     fn pre_compute(&mut self){
        for polygon in &mut  self.polygons{
            polygon.pre_compute();
        }
    }

    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        if !self.bounding_box.test_collision(ray){
            return None;
        }

        let potential_hit = self
            .polygons
            .iter()
            .map(|polygon| (polygon, polygon.get_distance(ray)))
            .filter(|(_polygon, hit)| hit.is_some())
            .map(|(polygon, hit)| (polygon, hit.unwrap()))
            .reduce(|accumulator, (polygon, (distance, hit))| {
                if distance < accumulator.1.0 {
                    (polygon, (distance, hit))
                } else {
                    accumulator
                }
            });
            if let Some((polygon, (distance,hit))) = potential_hit {
                return Some(Hit{
                    distance,
                    position: ray.at(distance),
                    normal: polygon.normal,
                    parallel_to_surface: polygon.parallel_to_surface,
                })
            }
            None
        
    }

}
impl MulAssign<f64> for Mesh {
    fn mul_assign(&mut self, rhs: f64) {
        self.polygons.iter_mut().for_each(|polygon| *polygon *= rhs);
        self.bounding_box *= rhs;
    }
}

impl AddAssign<Vec3> for Mesh {
    fn add_assign(&mut self, rhs: Vec3) {
        self.polygons.iter_mut().for_each(|polygon| *polygon += rhs);
        self.bounding_box += rhs;
    }
}
