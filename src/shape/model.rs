use std::{ops::{AddAssign, MulAssign}, fmt::Display};

use crate::{algebra::vec3::Vec3, hit::Hit, ray::Ray};

use super::{polygon::Polygon, cuboid::Cuboid, Shape};
#[derive(Debug)]
pub struct Model {
    pub polygons: Vec<Polygon>,
    pub bounding_box: Cuboid
}

impl Model {
    pub fn new(polygons: Vec<Polygon>) -> Self {
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
        Model { polygons, bounding_box: Cuboid{min_vertex: bounding_points[0], max_vertex: bounding_points[1]}}
    }
}

impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model with {} polygons, with bounds: {}", self.polygons.len(), self.bounding_box)
    }
}

impl Shape for Model{
    fn get_potential_hit(&self, ray: &Ray) -> Option<Hit> {
        if !self.bounding_box.test_collision(ray){
            return None;
        }

        let potential_hit = self
            .polygons
            .iter()
            .map(|polygon| (polygon, polygon.get_hit_distance(ray)))
            .filter(|(_polygon, distance)| distance.is_some())
            .map(|(polygon, distance)| (polygon, distance.unwrap()))
            .reduce(|accumulator, (polygon, distance)| {
                if distance < accumulator.1 {
                    (polygon, distance)
                } else {
                    accumulator
                }
            });

        if let Some((polygon, distance)) = potential_hit {
            return Some(Hit {
                distance,
                position: ray.at(distance),
                normal: polygon.normal,
                parallel_to_surface: polygon.parallel_to_surface,
            });
        }
        None
    }
}
impl MulAssign<f64> for Model {
    fn mul_assign(&mut self, rhs: f64) {
        self.polygons.iter_mut().for_each(|polygon| *polygon *= rhs);
        self.bounding_box *= rhs;
    }
}

impl AddAssign<Vec3> for Model {
    fn add_assign(&mut self, rhs: Vec3) {
        self.polygons.iter_mut().for_each(|polygon| *polygon += rhs);
        self.bounding_box += rhs;
    }
}
