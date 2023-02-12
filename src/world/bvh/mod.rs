mod bounding_box;



use crate::{algebra::{vec3::Vec3, axis::Axis}, algebra::ray::Ray};

use self::bounding_box::BoundingBox;

use super::triangle_hit_parser::TriangleHitParser;

pub struct BoundedVolume {
    bounding_box : BoundingBox,
    children: BoundedVolumeChildren,
}

pub enum BoundedVolumeChildren {
    BoundedVolumes([Box<BoundedVolume>; 2]),
    TriangleIndices([Option<usize>; 2]),
}

impl BoundedVolume {
    pub fn new(triangles: &Vec<TriangleHitParser>) -> Self {
        let indices : Vec<usize> = (0..triangles.len()).collect();
        let centroids = triangles
            .iter()
            .map(|triangle| triangle.get_center())
            .collect();
        Self::build(indices, &centroids, triangles)
    }

    pub fn build(indices: Vec<usize>, centroids: &Vec<Vec3>, triangles: &Vec<TriangleHitParser>) -> Self {
        let bounding_box = BoundingBox::new_from_triangle_indices(triangles, &indices);

        if indices.len() <= 2 {
            let children = 
                if indices.is_empty(){
                    BoundedVolumeChildren::TriangleIndices([None, None]) 
                }else if indices.len() == 1{
                    BoundedVolumeChildren::TriangleIndices([Some(indices[0]), None])
                } else{
                    BoundedVolumeChildren::TriangleIndices([Some(indices[0]), Some(indices[1])])
                };
            return Self {
                bounding_box,
                children,
            };
        }


        let centroid_bounding_box = BoundingBox::new_from_vec3_indices(centroids, &indices);
        let cbb_size = centroid_bounding_box.size();
        let split_axis = {
            if cbb_size.x > cbb_size.y && cbb_size.x > cbb_size.z { Axis::X }
            else if cbb_size.y > cbb_size.z {Axis::Y}
            else{ Axis::Z }
        };
        let split_value = indices.iter().map(|&index| centroids[index].axis(split_axis)).sum::<f64>() / indices.len() as f64;

        let mut sub_volume_indices : Vec<_> = indices
            .iter()
            .filter(|&&index| *centroids[index].axis(split_axis) < split_value)
            .copied()
            .collect();
        let mut super_volume_indices : Vec<_> = indices
            .iter()
            .filter(|&&index| *centroids[index].axis(split_axis) >= split_value)
            .copied()
            .collect();
        
        if sub_volume_indices.is_empty(){
            sub_volume_indices.append(&mut super_volume_indices.drain(0..1).collect());
        }        
        if super_volume_indices.is_empty(){
            super_volume_indices.append(&mut sub_volume_indices.drain(0..1).collect());
        }

        Self{
            bounding_box, 
            children: BoundedVolumeChildren::BoundedVolumes([
                Box::new(Self::build(sub_volume_indices, centroids, triangles)),
                Box::new(Self::build(super_volume_indices, centroids, triangles)),
                ]) 
        }

    }


    /// Calculates Surface Area Heuristic
    /// See https://jacco.ompf2.com/2022/04/18/how-to-build-a-bvh-part-2-faster-rays/
    // pub fn evaluate_SAH(split_position: f64, split_axis: Axis, indices: &Vec<usize>, centroids: &Vec<Vec3>) -> f64{
        
    //     let under_split_count = indices
    //         .iter()
    //         .map(|&index| *centroids[index].axis(split_axis))
    //         .filter(|&axis_centroid| axis_centroid < split_position)
    //         .count();
    // }

    pub fn get_intersecting_indices(&self, ray: &Ray) -> Vec<usize>{
        if !self.bounding_box.test_collision(ray){
            return vec![];
        }

        match &self.children{
            BoundedVolumeChildren::BoundedVolumes(volumes) => {
                volumes
                    .iter()
                    .flat_map(|volume| volume.get_intersecting_indices(ray))
                    .collect()
            },
            BoundedVolumeChildren::TriangleIndices(indices) => {
                indices
                    .iter()
                    .filter_map(|&index| index)
                    .collect()
            },
        }
    }
}
