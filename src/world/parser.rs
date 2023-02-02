use std::sync::{Arc, RwLock};

use regex::Regex;

use crate::{algebra::vec3::Vec3, material::Material};

use super::{model::Model, triangle::Triangle, Vertex, VertexNormal, World};

pub fn parse_ascii_stl(input: &str, world: &mut World) -> Model {
    let regex = Regex::new(
        r"facet normal\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*outer loop\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*endloop\s*endfacet",
    )
    .unwrap();

    world.materials.push(Arc::new(RwLock::new(Material::base_diffuse())));
    let mut model = Model {
        vertex_indexes: vec![],
        material: world.materials.last().unwrap().clone(),
    };

    for capture in regex.captures_iter(input) {
        world.vertices.push(Vertex(Vec3::new(
            str::parse::<f64>(&capture[4]).unwrap(),
            str::parse::<f64>(&capture[5]).unwrap(),
            str::parse::<f64>(&capture[6]).unwrap(),
        )));
        world.vertices.push(Vertex(Vec3::new(
            str::parse::<f64>(&capture[7]).unwrap(),
            str::parse::<f64>(&capture[8]).unwrap(),
            str::parse::<f64>(&capture[9]).unwrap(),
        )));
        world.vertices.push(Vertex(Vec3::new(
            str::parse::<f64>(&capture[10]).unwrap(),
            str::parse::<f64>(&capture[11]).unwrap(),
            str::parse::<f64>(&capture[12]).unwrap(),
        )));
        let triangle = Triangle {
            vertex_indexes: [
                world.vertices.len() - 1,
                world.vertices.len() - 2,
                world.vertices.len() - 3,
            ],
            normal: Vec3::new(
                str::parse::<f64>(&capture[1]).unwrap(),
                str::parse::<f64>(&capture[2]).unwrap(),
                str::parse::<f64>(&capture[3]).unwrap(),
            ),
            material: world.materials.last().unwrap().clone(),
            vertex_normal_indexes: None,
            vertex_color_indexes: None,
        };
        world.triangles.push(triangle);
        model.vertex_indexes.push(world.triangles.len() - 1);
        model.vertex_indexes.push(world.triangles.len() - 2);
        model.vertex_indexes.push(world.triangles.len() - 3);
    }
    model
}

pub fn parse_ascii_obj(input: &str, world: &mut World) -> Model {
    let material = Arc::new(RwLock::new(Material::base_diffuse()));
    world.materials.push(material.clone());

    let starting_vertices_index = world.vertices.len();
    let starting_vertex_normals_index = world.vertex_normals.len();

    let vertex_regex = Regex::new(r"v\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)").unwrap();
    for capture in vertex_regex.captures_iter(input) {
        world.vertices.push(Vertex(Vec3::new(
            str::parse::<f64>(&capture[1]).unwrap_or_else(|err| {
                println!("{}, {}", err, &capture[1]);
                1.
            }),
            str::parse::<f64>(&capture[2]).unwrap(),
            str::parse::<f64>(&capture[3]).unwrap(),
        )));
    }

    let vertex_normal_regex =
        Regex::new(r"vn\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)").unwrap();
    for capture in vertex_normal_regex.captures_iter(input) {
        world.vertex_normals.push(VertexNormal(Vec3::new(
            str::parse::<f64>(&capture[1]).unwrap(),
            str::parse::<f64>(&capture[2]).unwrap(),
            str::parse::<f64>(&capture[3]).unwrap(),
        )));
    }

    let triangle_regex = Regex::new(r"f\s+([\w]+)\s+([\w]+)\s+([\w]+)").unwrap();
    for capture in triangle_regex.captures_iter(input) {
        let vertex_indexes = [
            str::parse::<usize>(&capture[1]).unwrap() + starting_vertices_index - 1,
            str::parse::<usize>(&capture[2]).unwrap() + starting_vertices_index - 1,
            str::parse::<usize>(&capture[3]).unwrap() + starting_vertices_index - 1,
        ];

        let vertex_normal_indexes = [
            str::parse::<usize>(&capture[1]).unwrap() + starting_vertex_normals_index - 1,
            str::parse::<usize>(&capture[2]).unwrap() + starting_vertex_normals_index - 1,
            str::parse::<usize>(&capture[3]).unwrap() + starting_vertex_normals_index - 1,
        ];

        let mut normal = Vec3::cross(
            &(world.vertices[vertex_indexes[1]].0 - world.vertices[vertex_indexes[0]].0),
            &(world.vertices[vertex_indexes[2]].0 - world.vertices[vertex_indexes[0]].0),
        );
        if normal.dot(&world.vertex_normals[vertex_normal_indexes[0]].0) < 0. {
            normal *= -1.;
        }

        let triangle = Triangle {
            vertex_indexes,
            normal,
            material: material.clone(),
            vertex_normal_indexes: Some(vertex_normal_indexes),
            vertex_color_indexes: None,
        };
        world.triangles.push(triangle);
    }

    Model {
        vertex_indexes: (starting_vertices_index..world.vertex_normals.len() - 1).collect(),
        material: world.materials.last().unwrap().clone(),
    }
}
