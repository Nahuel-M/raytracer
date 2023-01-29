use std::fmt::Display;

mod phong_triangle;
mod phong_vertex;

use crate::{ray::Ray, hit::Hit};

use self::{phong_vertex::PhongVertex, phong_triangle::PhongTriangle};

use super::{Shape, cuboid::Cuboid};

pub struct PhongMesh<'a>{
    vertices : Vec<PhongVertex>,
    polygons : Vec<PhongTriangle<'a>>,
    bounding_box: Cuboid,
    last_hit: Option<&'a PhongTriangle<'a>>,
}

impl Display for PhongMesh<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model with {} polygons, with bounds: {}", self.polygons.len(), self.bounding_box)
    }
}

impl Shape for PhongMesh<'_>{
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
    fn pre_compute(&mut self){
        for polygon in &mut  self.polygons{
            polygon.pre_compute();
        }
    }

    fn get_distance(&mut self, ray: &crate::ray::Ray) -> Option<f64> {
        todo!()
    }
}



