use crate::{algebra::{ray::Ray, vec3::Vec3}, hit::Hit};

use super::bvh::BoundedVolume;
// #[derive(Default)]
pub struct PreComputedWorld<'a>{
    pub bounded_volume_hierarchy: BoundedVolume<'a>,
}


impl PreComputedWorld<'_>{
    pub fn potential_hit(&self, ray: &Ray) -> Option<Hit> {
        // Check potential matches using BVH
        let potential_collision = self.bounded_volume_hierarchy.get_potential_collision(ray);
        potential_collision?;

        // If an intersection is found, construct a hit
        if let Some((triangle, distance)) = potential_collision {
            let (a, b) = triangle.get_barycentric_a_b(ray);
            let barycentrics = [1. - a - b, a, b, ];

            let mut normal = triangle.vertex_normals
                .iter()
                .zip(barycentrics.iter())
                .map(|(&normal, &barycentric)| normal * barycentric)
                .sum::<Vec3>()
                .normalize();
           
           if normal.dot(&triangle.normal) < 0.{
                normal = triangle.normal;
           }

            return Some(Hit {
                distance,
                position: ray.at(distance),
                normal,
                material: triangle.material,
            });
        }
        None
    }
}