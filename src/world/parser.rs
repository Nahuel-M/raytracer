use regex::Regex;

use crate::{algebra::vec3::Vec3, material::Material};

use super::{triangle::Triangle, Vertex, World};
pub trait Parser {
    fn parse_ascii_stl<'a>(input: &str, world: &mut World<'a>) {
        let regex = Regex::new(
        r"facet normal\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*outer loop\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*endloop\s*endfacet",
    )
    .unwrap();

        world.materials.push(Material::base_diffuse());

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
            world.triangles.push(Triangle {
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
                material: &world.materials.last().unwrap(),
                vertex_normals: None,
                vertex_colors: None,
            })
        }
    }
}

impl Parser for World{
    
}