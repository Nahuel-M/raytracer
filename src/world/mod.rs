use std::{fmt::Display, fs, sync::{Arc, RwLock}};
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

pub struct World {
    pub camera: Camera,
    pub background: Vec3,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub triangle_hit_parsers: Vec<TriangleHitParser>,
    pub materials: Vec<Arc<RwLock<Material>>>,
    pub vertex_normals: Vec<VertexNormal>,
    // pub models: Vec<Model>,
}

impl<'a> World {
    pub fn with_camera(camera: Camera) -> Self {
        World {
            camera,
            background: Vec3::zeros(),
            vertices: vec![],
            triangles: vec![],
            triangle_hit_parsers: vec![],
            materials: vec![],
            vertex_normals: vec![],
            // models: vec![],
        }
    }

    pub fn insert_model_by_filename(&'a mut self, filename: &str) -> Model {
        let (_name, extension) = filename.split_once('.').unwrap_or(("", ""));
        let file_string = fs::read_to_string(filename).unwrap();
        match extension {
            "stl" => parser::parse_ascii_stl(file_string.as_str(), self),
            "obj" => parser::parse_ascii_obj(file_string.as_str(), self),
            _ => Model{ vertex_indexes: vec![], material: Arc::new(RwLock::new(Material::base_diffuse())) }
        }
    }

    pub fn get_ray_collision(&self, ray: &Ray) -> Option<Hit> {
        let potential_distance = self
            .triangle_hit_parsers
            .iter()
            .map(|triangle| triangle.get_hit_distance(ray))
            .enumerate()
            .filter(|(_index, distance)| distance.is_some())
            .map(|(index, distance)| (index, distance.unwrap()))
            .reduce(|accumulator, (index, distance)| {
                if distance < accumulator.1 {
                    (index, distance)
                } else {
                    accumulator
                }
            });
        if let Some((index, distance)) = potential_distance {
            let triangle = &self.triangles[index];
            let mut normal = triangle.normal;
            if let Some(normal_indexes) = triangle.vertex_normal_indexes{
                let (a, b) = &self.triangle_hit_parsers[index].get_barycentric_a_b(ray);
                let barycentrics = [1. - a - b, *a, *b, ];

                normal = normal_indexes
                    .iter()
                    .map(|&normal_index| &self.vertex_normals[normal_index].0)
                    .zip(barycentrics.iter())
                    .map(|(&normal, &barycentric)| normal * barycentric)
                    .sum::<Vec3>()
                    .normalize();
            }

            return Some(Hit {
                distance,
                position: ray.at(distance),
                normal,
                parallel_to_surface: (self.vertices[triangle.vertex_indexes[0]].0
                    - self.vertices[triangle.vertex_indexes[1]].0)
                    .normalize(),
                material: Some(triangle.material.clone()),
            });
        }
        None
    }

    pub fn pre_compute(&mut self) {
        self.triangle_hit_parsers.clear();

        for triangle in self.triangles.iter() {
            self.triangle_hit_parsers
                .push(triangle.generate_hit_parser(&self.vertices))
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "World with {} vertices, {} triangles and {} materials",
            self.vertices.len(),
            self.triangles.len(),
            self.materials.len()
        )
        .unwrap();
        writeln!(f, "{}", self.camera).unwrap();
        writeln!(f)
    }
}
