mod bounding_box;

use crate::{algebra::{vec3::Vec3, axis::Axis}, algebra::ray::Ray};

use self::bounding_box::{BoundingBox, Union};

use super::triangle_hit_parser::TriangleHitParser;
pub struct BoundedVolume<'a> {
    bounding_box : BoundingBox,
    children: BoundedVolumeChildren<'a>,
}

pub enum BoundedVolumeChildren<'a> {
    BoundedVolumes([Box<BoundedVolume<'a>>; 2]),
    Triangles(Vec<TriangleHitParser<'a>>),
}

impl<'a> BoundedVolume<'a> {
    pub fn new(triangles: Vec<TriangleHitParser<'a>>) -> Self {
        Self::build(triangles, 5)
    }

    pub fn build(triangles: Vec<TriangleHitParser<'a>>, resolution: usize) -> BoundedVolume<'a>{
        let centroids: Vec<Vec3> = triangles.iter().map(TriangleHitParser::get_center).collect();
        let centroid_bounding_box = BoundingBox::new_from_vec3s(&centroids);
        let own_bounding_box = BoundingBox::new_from_triangles(&triangles);
        let own_sah = own_bounding_box.area() * triangles.len() as f64;

        let mut lowest_sah : (f64, Axis, usize) = (f64::MAX, Axis::X, 0);
        for axis in Axis::ALL{
            let mut bins : Vec<Vec<&TriangleHitParser>> = (0..resolution).map(|_| Vec::new()).collect();
            let min = centroid_bounding_box.minimums.axis(axis);
            let max = centroid_bounding_box.maximums.axis(axis);
            let step_size = (max - min) / resolution as f64;

            // Find correct bin for each triangle
            for (centroid, triangle) in centroids.iter().zip(&triangles){
                let bin = usize::min(
                    ((centroid.axis(axis) - min) / step_size) as usize, 
                    resolution - 1
                );
                bins[bin].push(triangle);
            }
            
            // Find lowest Surface Area Heuristic for each cut-off bin.
            let boxes : Vec<BoundingBox> = bins.iter().map(|triangles| BoundingBox::new_from_triangle_refs(triangles)).collect();
            for step in 1..(resolution-1){
                let left_box = (&boxes[0..step]).union();
                let right_box = (&boxes[step..resolution]).union();
                let sah = left_box.area() * bins[0..step].iter().map(|bin| bin.len()).sum::<usize>() as f64
                        + right_box.area() * bins[step..resolution].iter().map(|bin| bin.len()).sum::<usize>() as f64;
                if sah < lowest_sah.0{
                    lowest_sah = (sah, axis, step);
                }
            }
        }

        if own_sah <= lowest_sah.0 + 1e-5 || triangles.len() < 2{
            BoundedVolume{
                bounding_box: own_bounding_box,
                children: BoundedVolumeChildren::Triangles(triangles)
            }
        } else {
            let min = centroid_bounding_box.minimums.axis(lowest_sah.1);
            let max = centroid_bounding_box.maximums.axis(lowest_sah.1);
            let step_size = (max - min) / resolution as f64;
            let split = min + step_size * lowest_sah.2 as f64;

            let (triangles1, triangles2) : (Vec<TriangleHitParser>, Vec<TriangleHitParser>)= triangles.into_iter()
                .partition(|&triangle| *triangle.get_center().axis(lowest_sah.1) < split);
            BoundedVolume{
                bounding_box: own_bounding_box,
                children: BoundedVolumeChildren::BoundedVolumes([
                    Box::new(BoundedVolume::build(triangles1, resolution)),
                    Box::new(BoundedVolume::build(triangles2, resolution)),
                ])
            }
        }

    }

    pub fn get_potential_collision(&self, ray: &Ray) -> Option<(&TriangleHitParser, f64)> {
        let bounding_box_distance = self.bounding_box.distance(ray);
        bounding_box_distance?;

        self.traverse_children(ray, & (1. / ray.direction_unit))
    }

    #[inline]
    pub fn traverse_children(&self, ray: &Ray, inverse_ray_direction: &Vec3) -> Option<(&TriangleHitParser, f64)>{
        match &self.children{
            BoundedVolumeChildren::BoundedVolumes(volumes) => {
                let distance_0 = volumes[0].bounding_box.optimized_distance(&ray.origin, inverse_ray_direction);
                let distance_1 = volumes[1].bounding_box.optimized_distance(&ray.origin, inverse_ray_direction);

                match (distance_0, distance_1){
                    (None, None) => None,
                    (Some(_), None) => volumes[0].traverse_children(ray, inverse_ray_direction),
                    (None, Some(_)) => volumes[1].traverse_children(ray, inverse_ray_direction),
                    // In case both bounding boxes are hit:
                    (Some(distance_0), Some(distance_1)) => {
                        let (close, far, far_distance) = 
                            if distance_0 > distance_1{
                                (&volumes[1], &volumes[0], distance_0)
                            } else {
                                (&volumes[0], &volumes[1], distance_1)
                            };
                        // We traverse the closest box first, and if the ray doesn't hit anything, we traverse the far one.
                        if let Some((triangle_0, distance_0)) = close.traverse_children(ray, inverse_ray_direction) {
                            // If the far box is closer than the hit in the close box, we traverse the far box.
                            if far_distance < distance_0{
                                if let Some((triangle_1, distance_1)) = far.traverse_children(ray, inverse_ray_direction){
                                    if distance_1 < distance_0{
                                        return Some((triangle_1, distance_1))
                                    } 
                                } 
                            } 
                            return Some((triangle_0, distance_0))
                        }
                        return far.traverse_children(ray, inverse_ray_direction)
                    }
                }
            },
            BoundedVolumeChildren::Triangles(triangles) => {
                triangles
                    .iter()
                    .map(|triangle| (triangle, triangle.get_hit_distance(ray)))
                    .filter(|(_triangle, distance)| distance.is_some())
                    .map(|(triangle, distance)| (triangle, distance.unwrap()))
                    .reduce(|(closest_triangle, closest_distance), (new_triangle, new_distance)| 
                        if closest_distance > new_distance{
                            (new_triangle, new_distance)
                        } else {
                            (closest_triangle, closest_distance)
                        }
                     )
            }
        }
    }
}