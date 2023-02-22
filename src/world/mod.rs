use std::{fmt::Display, fs, collections::HashMap};
pub mod camera;
pub mod model;
pub mod vertex;
pub mod triangle;
mod parser;

use crate::{algebra::vec3::Vec3, material::{map::RgbMap, Material}};

use self::{
    camera::Camera, model::Model,
};


type VertexNormal = Vec3;
#[allow(dead_code)]
pub struct VertexColor(Vec3);

pub struct World {
    pub camera: Camera,
    pub background: RgbMap,
    pub materials: HashMap<String, Material>,
    pub models: HashMap<String, Model>,
    pub vertex_normals: Vec<VertexNormal>,
}

impl<'a> World {
    pub fn with_camera(camera: Camera) -> Self {
        World {
            camera,
            background: RgbMap::Color(Vec3::ZEROS),
            models: HashMap::new(),
            vertex_normals: vec![],
            materials: HashMap::from([("base_diffuse".to_string(), Material::base_diffuse())]),
        }
    }
    pub fn import_3d_file(&'a mut self, filename: &str) -> Result<String, String> {
        let (_name, extension) = filename.split_once('.').unwrap_or(("", ""));
        let file_string = fs::read_to_string(filename).unwrap();
        match extension {
            // "stl" => parser::parse_ascii_stl(file_string.as_str(), self, model_name),
            "obj" => parser::parse_ascii_obj(file_string.as_str(), self)?,
            _ => Err("Extension not supported")?,
        };
        Ok("Success.".to_string())
    }

    pub fn import_material_file(&'a mut self, filename: &str) -> Result<String, String> {
        let (_name, extension) = filename.split_once('.').unwrap_or(("", ""));
        let file_string = fs::read_to_string(filename).unwrap();
        match extension {
            // "stl" => parser::parse_ascii_stl(file_string.as_str(), self, model_name),
            "mtl" => parser::parse_mtl(file_string.as_str(), self)?,
            _ => Err("Extension not supported")?,
        };
        Ok("Success.".to_string())
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "World with {} models",
            self.models.len()
        )
        .unwrap();
        writeln!(f, "{}", self.camera).unwrap();
        writeln!(f)
    }
}
