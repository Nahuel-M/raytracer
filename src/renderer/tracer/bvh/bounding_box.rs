use packed_simd_2::f64x4;

use crate::{algebra::{vec3::Vec3, axis::Axis}, algebra::ray::Ray, renderer::tracer::triangle_hit_parser::TriangleHitParser};
#[derive(Default, Clone, Copy)]
pub struct BoundingBox {
    pub minimums: Vec3,
    pub maximums: Vec3,
}
#[allow(dead_code)]
impl BoundingBox {
    pub fn new_from_vec3s(points: &[Vec3]) -> Self{
        if points.is_empty(){
            return Self{minimums: Vec3::ZEROS, maximums: Vec3::ZEROS};
        }
        let mut bounding_box = Self{ minimums: points[0], maximums: points[0] };
        for vec3 in points{
            bounding_box.grow_with_vec3(vec3);
        }
        bounding_box
    }

    pub fn new_from_vec3_indices(points: &[Vec3], indices: &[usize]) -> Self{
        if indices.is_empty() || points.is_empty(){
            return Self{minimums: Vec3::ZEROS, maximums: Vec3::ZEROS};
        }
        let mut bounding_box = Self{ minimums: points[indices[0]], maximums: points[indices[0]] };
        for index in indices{
            bounding_box.grow_with_vec3(&points[*index]);
        }
        bounding_box
    }
 
    pub fn new_from_triangles(triangles: &[TriangleHitParser]) -> Self{
        if triangles.is_empty(){
            return Self{minimums: Vec3::ZEROS, maximums: Vec3::ZEROS};
        }
        let mut bounding_box = Self{ minimums: triangles[0].vertices[0], maximums: triangles[0].vertices[0] };
        for triangle in triangles{
            bounding_box.grow_with_triangle(triangle);
        }
        bounding_box
    }

    pub fn new_from_triangle_refs(triangles: &[&TriangleHitParser]) -> Self{
        if triangles.is_empty(){
            return Self{minimums: Vec3::ZEROS, maximums: Vec3::ZEROS};
        }
        let mut bounding_box = Self{ minimums: triangles[0].vertices[0], maximums: triangles[0].vertices[0] };
        for triangle in triangles{
            bounding_box.grow_with_triangle(triangle);
        }
        bounding_box
    }

    pub fn new_from_triangle_indices(triangles: &[TriangleHitParser], indices: &[usize]) -> Self{
        if indices.is_empty() || triangles.is_empty(){
            return Self{minimums: Vec3::ZEROS, maximums: Vec3::ZEROS};
        }
        let mut bounding_box = Self{ minimums: triangles[indices[0]].vertices[0], maximums: triangles[indices[0]].vertices[0] };
        for index in indices{
            bounding_box.grow_with_triangle(&triangles[*index]);
        }
        bounding_box
    }

    pub fn union(&self, other: &Self) -> Self{
        Self{
            minimums: Vec3::ew_min(&self.minimums, &other.minimums),
            maximums: Vec3::ew_max(&self.maximums, &other.maximums)
        }
    }

    pub fn grow_with_vec3(&mut self, vec3: &Vec3) {
        self.minimums = self.minimums.ew_min(vec3);
        self.maximums = self.maximums.ew_max(vec3);
    }

    pub fn grow_with_triangle(&mut self, triangle: &TriangleHitParser){
        for vertex in triangle.vertices{
            self.minimums = self.minimums.ew_min(&vertex);
            self.maximums = self.maximums.ew_max(&vertex);
        }
    }

    pub fn size(&self) -> Vec3 {
        self.maximums - self.minimums
    }

    pub fn area(&self) -> f64 {
        let size = self.size();
        size.x * size.y * 2. + size.y * size.z * 2. + size.z * size.x * 2.
    }

    pub fn test_collision(&self, ray : &Ray) -> bool{
        let min_vertex = self.minimums - ray.origin;
        let max_vertex = self.maximums - ray.origin;

        let distances_to_min_vertex = min_vertex / ray.direction_unit;
        let distances_to_max_vertex = max_vertex / ray.direction_unit;

        let min_distance = distances_to_min_vertex.ew_min(&distances_to_max_vertex);
        let max_distance = distances_to_min_vertex.ew_max(&distances_to_max_vertex);

        min_distance.max() <= max_distance.min()
    }

    pub fn distance(&self, ray : &Ray) -> Option<f64>{
        let min_vertex = self.minimums - ray.origin;
        let max_vertex = self.maximums - ray.origin;

        let distances_to_min_vertex = min_vertex / ray.direction_unit;
        let distances_to_max_vertex = max_vertex / ray.direction_unit;

        let min_distance = distances_to_min_vertex.ew_min(&distances_to_max_vertex);
        let max_distance = distances_to_min_vertex.ew_max(&distances_to_max_vertex);

        if min_distance.max() <= max_distance.min() {
            Some(min_distance.max())
        } else {
            None
        }
    }
    #[inline]
    pub fn optimized_distance(&self, ray_origin: &Vec3, inverted_ray_direction: &Vec3) -> Option<f64>{
        let minimums = f64x4::new(self.minimums.x, self.minimums.y, self.minimums.z, 0.);
        let maximums = f64x4::new(self.maximums.x, self.maximums.y, self.maximums.z, f64::MAX / 2.);
        let ray_origin = f64x4::new(ray_origin.x, ray_origin.y, ray_origin.z, 0.);
        let inverted_ray_direction = f64x4::new(inverted_ray_direction.x, inverted_ray_direction.y, inverted_ray_direction.z, 1.);
  
        let min_vertex = minimums - ray_origin;
        let max_vertex = maximums - ray_origin;

        let distances_to_min_vertex = min_vertex * inverted_ray_direction;
        let distances_to_max_vertex = max_vertex * inverted_ray_direction;

        let min_distance = f64x4::min(distances_to_min_vertex, distances_to_max_vertex);
        let max_distance = f64x4::max(distances_to_min_vertex, distances_to_max_vertex);

        if min_distance.max_element() <= max_distance.min_element() {
            Some(min_distance.max_element())
        } else {
            None
        }
    }

    pub fn split_at(&self, axis: Axis, position : f64) -> (BoundingBox, BoundingBox){
        let mut bb1 = *self;
        let mut bb2 = *self;
        *bb1.maximums.axis_mut(axis) = position;
        *bb2.minimums.axis_mut(axis) = position;
        (bb1, bb2)
    }
}

pub trait Union{
    fn union(&self) -> BoundingBox
        where Self : Sized;
}

impl Union for &[BoundingBox]{
    fn union(&self) -> BoundingBox
        where Self : Sized {
        self.iter().copied().reduce(|acc, e| acc.union(&e)).unwrap()
    }
}
