use crate::{
    hit::TraceResult,
    world::{triangle::Triangle, World, model::{Vertex, Model}}, algebra::{ray::Ray, vec3::Vec3},
};

use self::{
    bvh::BoundedVolume,
    pre_computed_world::{PreComputedWorld, TriangleRemainingData},
    triangle_hit_parser::TriangleHitParser,
};

mod bvh;
mod pre_computed_world;
mod triangle_hit_parser;
pub mod trace_package;

#[derive(Default)]
pub struct Tracer<'a> {
    pre_computed_world: PreComputedWorld<'a>,
}

impl<'a> Tracer<'a> {
    pub fn pre_compute(&mut self, world: &'a World) {
        let triangle_hit_parsers = world.models
            .iter()
            .flat_map(|(_name, model)| {
                model.faces.iter().map(|triangle| Tracer::hit_parser_from_triangle(triangle, &model.vertices))
            })
            .collect();

        let triangle_remaining_data = world.models
            .iter()
            .flat_map(|(_name, model)| {
                model.faces.iter().map(|triangle| Tracer::remaining_data_from_triangle(world, model, triangle))
            })
            .collect();

        let bounded_volume_hierarchy = BoundedVolume::new(&triangle_hit_parsers);
        self.pre_computed_world = PreComputedWorld {
            triangle_hit_parsers,
            triangle_remaining_data,
            bounded_volume_hierarchy,
        };
        println!("Finished precompute with {} faces", self.pre_computed_world.triangle_hit_parsers.len());
    }

    fn remaining_data_from_triangle(world: &'a World, model: &Model, triangle: &Triangle) -> TriangleRemainingData<'a>{

        let vertex_normals : [Vec3; 3] = 
            if triangle.smoothing && triangle.vertex_normals.is_some(){
                triangle.vertex_normals.unwrap().iter()
                        .map(|&index| model.vertex_normals[index])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
            } else{
                [triangle.normal; 3]
            };

        let material = world.materials.get(&model.material_name).unwrap();
        TriangleRemainingData { vertex_normals, material }
    }

    pub fn trace_ray(&self, ray: &Ray) -> TraceResult {
        let potential_hit = self.pre_computed_world.potential_hit(ray);
        if let Some(hit) = potential_hit {
            TraceResult::Hit(hit)
        } else {
            TraceResult::Miss
        }
    }

    pub(crate) fn clear(&mut self){
        self.pre_computed_world = Default::default();
    }

    fn hit_parser_from_triangle(triangle: &Triangle, vertices: &[Vertex]) -> TriangleHitParser{
        let vertices : [Vec3; 3] = triangle.vertices
            .iter()
            .map(|&index| vertices[index])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let edge_1 = vertices[1] - vertices[0];
        let edge_2 = vertices[2] - vertices[0];

        let v1 = edge_1 - edge_1.project(&edge_2);
        let v2 = edge_2 - edge_2.project(&edge_1);

        let inv_proj_1 = 1. / edge_1.dot(&v1);
        let inv_proj_2 = 1. / edge_2.dot(&v2);

        TriangleHitParser {
            normal: triangle.normal,
            vertices,
            inv_proj_1,
            inv_proj_2,
            v1,
            v2,
        }
    }
}
