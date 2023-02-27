use crate::{algebra::{ray::Ray, vec3::Vec3}, hit::Hit, material::Material};

use super::{triangle_hit_parser::TriangleHitParser, bvh::BoundedVolume};
#[derive(Default)]
pub struct PreComputedWorld<'a>{
    pub triangle_hit_parsers: Vec<TriangleHitParser>,
    pub triangle_remaining_data: Vec<TriangleRemainingData<'a>>,
    pub bounded_volume_hierarchy: BoundedVolume,
}

pub struct TriangleRemainingData<'a>{
    pub vertex_normals: [Vec3; 3],
    pub material: &'a  Material
}

impl PreComputedWorld<'_>{
    pub fn potential_hit(&self, ray: &Ray) -> Option<Hit> {
        // Check potential matches using BVH
        let potential_indices = self.bounded_volume_hierarchy.get_intersecting_indices(ray);
        if potential_indices.is_empty(){
            return None;
        }
        // For each potential match, perform full triangle ray intersection. Find closest to ray origin
        let potential_distance = potential_indices
            .iter()
            .map(|&index| (index, self.triangle_hit_parsers[index].get_hit_distance(ray)))
            .filter(|(_index, distance)| distance.is_some())
            .map(|(index, distance)| (index, distance.unwrap()))
            .reduce(|accumulator, (index, distance)| {
                if distance < accumulator.1 {
                    (index, distance)
                } else {
                    accumulator
                }
            });

        // If an intersection is found, construct a hit
        if let Some((index, distance)) = potential_distance {
            let (a, b) = &self.triangle_hit_parsers[index].get_barycentric_a_b(ray);
            let barycentrics = [1. - a - b, *a, *b, ];

            let normal = self.triangle_remaining_data[index].vertex_normals
                .iter()
                .zip(barycentrics.iter())
                .map(|(&normal, &barycentric)| normal * barycentric)
                .sum::<Vec3>()
                .normalize();
           
            return Some(Hit {
                distance,
                position: ray.at(distance),
                normal,
                material: self.triangle_remaining_data[index].material,
            });
        }
        None
    }
}