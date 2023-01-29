use std::fmt::Display;

use crate::{algebra::vec3::Vec3, hit::Hit, shape::Shape, ray::Ray};

use super::phong_vertex::PhongVertex;

pub(crate) struct PhongTriangle<'a>{
    vertices: [&'a PhongVertex; 3],
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
    edge_1: Vec3,
    edge_2: Vec3,
    v_1: Vec3,
    v_2: Vec3
}

impl Display for PhongTriangle<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Polygon with normal: ".to_string() + self.normal.to_string().as_str())
    }
}

impl PhongTriangle<'_>{
    pub fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let distance_to_plane = self.normal.dot(&(self.vertices[0].position - ray.origin))   // 5+, 3*
            / self.normal.dot(&ray.direction_unit);                                 // 2+, 3*, 1/
        let point_on_plane = ray.at(distance_to_plane);                             // 3+, 3*

        let a = 1. - (self.vertices[1].position - point_on_plane).dot(&self.v_1) / self.edge_1.dot(&self.v_1);      // 7+, 6*, 1/
        if !(0. ..=1.).contains(&a){
            return None;
        }

        let b = 1. - (self.vertices[2].position - point_on_plane).dot(&self.v_2) / self.edge_2.dot(&self.v_2);       // 7+, 6*, 1/
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }
}

impl Shape for PhongTriangle<'_>{
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
    
    fn pre_compute(&mut self){
        self.edge_1 = self.vertices[1].position - self.vertices[0].position;
        self.edge_2 = self.vertices[2].position - self.vertices[0].position;

        self.v_1 = self.edge_1 - self.edge_1.project(&self.edge_2);  
        self.v_2 = self.edge_2 - self.edge_2.project(&self.edge_1);    
    }

    fn get_distance(&mut self, ray: &crate::ray::Ray) -> Option<f64> {
        let distance_to_plane = self.normal.dot(&(self.vertices[0].position - ray.origin))   
        / self.normal.dot(&ray.direction_unit);                         
        let point_on_plane = ray.at(distance_to_plane);                 

        let a = 1. - (self.vertices[1].position - point_on_plane).dot(&self.v_1) / self.edge_1.dot(&self.v_1);
        if !(0. ..=1.).contains(&a){
            return None;
        }

        let b = 1. - (self.vertices[2].position - point_on_plane).dot(&self.v_2) / self.edge_2.dot(&self.v_2);
        if !(0. ..=1.).contains(&b) || a + b > 1. {
            return None;
        }

        Some(distance_to_plane)
    }
}