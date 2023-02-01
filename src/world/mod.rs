use std::{fmt::Display, fs};
pub mod camera;
pub mod model;
mod parser;
pub mod triangle;
mod triangle_hit_parser;

use crate::{algebra::vec3::Vec3, hit::Hit, material::Material, ray::Ray};

use self::{
    camera::Camera, model::Model, triangle::Triangle, triangle_hit_parser::TriangleHitParser,
};

pub struct Vertex(Vec3);
pub struct VertexNormal(Vec3);
pub struct VertexColor(Vec3);

pub struct World<'a> {
    pub camera: Camera,
    pub background: Vec3,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle<'a>>,
    pub triangle_hit_parsers: Vec<TriangleHitParser>,
    pub materials: Vec<Material>,
    pub vertex_normals: Vec<VertexNormal>,
    pub models: Vec<Model>,
}

impl<'a> World<'a> {
    pub fn with_camera(camera: Camera) -> Self {
        World {
            camera,
            background: Vec3::zeros(),
            vertices: vec![],
            triangles: vec![],
            triangle_hit_parsers: vec![],
            materials: vec![],
            vertex_normals: vec![],
            models: vec![],
        }
    }

    pub fn insert_model_by_filename<>(&'a mut self, filename: &str) {
        let (_name, extension) = filename.split_once('.').unwrap_or(("", ""));
        match extension {
            "stl" => {
                parser::parse_ascii_stl(
                    fs::read_to_string(filename).unwrap().as_str(),
                    self
                    
                );
            }
            _ => {},
        }
    }

    pub fn get_ray_collision(&self, ray: &Ray) -> Option<Hit> {
        let potential_distance = self
            .triangles
            .iter()
            .map(|triangle| (triangle, triangle.get_hit_distance(&self.vertices, ray)))
            .filter(|(_triangle, distance)| distance.is_some())
            .map(|(triangle, distance)| (triangle, distance.unwrap()))
            .reduce(|accumulator, (triangle, distance)| {
                if distance < accumulator.1 {
                    (triangle, distance)
                } else {
                    accumulator
                }
            });
        if let Some((triangle, distance)) = potential_distance {
            return Some(Hit {
                distance,
                position: ray.at(distance),
                normal: triangle.normal,
                parallel_to_surface: 
                    (self.vertices[triangle.vertex_indexes[0]].0
                    - self.vertices[triangle.vertex_indexes[1]].0)
                    .normalize(),
                material: triangle.material,
            });
        }
        None
    }

    pub fn pre_compute(&mut self){
        self.triangle_hit_parsers.clear();

        for triangle in self.triangles.iter(){
            self.triangle_hit_parsers.push(
                triangle.generate_hit_parser(&self.vertices)
            )
        }
    }
}

impl Display for World<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.camera).unwrap();
        for model in &self.models {
            writeln!(f, "{}", model).unwrap();
        }
        writeln!(f)
    }
}
