use crate::{
    hit::TraceResult,
    world::{triangle::Triangle, World, model::{Vertex, VertexNormal}}, algebra::{ray::Ray, vec3::Vec3}, material::Material,
};

use self::{
    bvh::BoundedVolume,
    pre_computed_world::PreComputedWorld,
    triangle_hit_parser::TriangleHitParser,
};

mod bvh;
mod pre_computed_world;
mod triangle_hit_parser;
pub mod trace_package;

#[derive(Default)]
pub struct Tracer<'a> {
    pre_computed_world: Option<PreComputedWorld<'a>>,
}

impl<'a> Tracer<'a> {
    pub fn pre_compute(&mut self, world: &'a World) {
        let triangle_hit_parsers : Vec<TriangleHitParser> = world.models
            .iter()
            .flat_map(|(_name, model)| {
                let material = world.materials.get(&model.material_name).unwrap();
                model.faces.iter().map(|triangle| Tracer::hit_parser_from_triangle(triangle, &model.vertices, &model.vertex_normals, material))
            })
            .collect();

        let face_count = triangle_hit_parsers.len();
        let bounded_volume_hierarchy = BoundedVolume::new(triangle_hit_parsers);
        self.pre_computed_world = Some(PreComputedWorld {
            // triangle_remaining_data,
            bounded_volume_hierarchy,
        });
        println!("Finished precompute with {} faces", face_count);
    }


    pub fn trace_ray(&self, ray: &Ray) -> TraceResult {
        // Todo: gracefully handle option
        let potential_hit = self.pre_computed_world.as_ref().unwrap().potential_hit(ray);
        if let Some(hit) = potential_hit {
            TraceResult::Hit(hit)
        } else {
            TraceResult::Miss
        }
    }

    pub(crate) fn clear(&mut self){
        self.pre_computed_world = Default::default();
    }

    fn hit_parser_from_triangle(triangle: &Triangle, vertices: &[Vertex], vertex_normals: &[VertexNormal], material: &'a Material) -> TriangleHitParser<'a>{
        let vertices : [Vec3; 3] = triangle.vertices
            .iter()
            .map(|&index| vertices[index])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let vertex_normals : [Vec3; 3] = 
            match triangle.vertex_normals{
                Some(indices) => {
                    indices
                    .iter()
                    .map(|&index| vertex_normals[index])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
                },
                None => [triangle.normal; 3],
            };
            

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
            vertex_normals,
            material,
        }
    }

}
