use crate::{algebra::vec3::Vec3, world::triangle_hit_parser::TriangleHitParser, ray::Ray};

pub struct BoundingBox {
    pub minimums: Vec3,
    pub maximums: Vec3,
}
#[allow(dead_code)]
impl BoundingBox {
    pub fn new_from_vec3s(points: &[Vec3]) -> Self{
        if points.is_empty(){
            return Self{minimums: Vec3::zeros(), maximums: Vec3::zeros()};
        }
        let mut bounding_box = Self{ minimums: points[0], maximums: points[0] };
        for vec3 in points{
            bounding_box.grow_with_vec3(vec3);
        }
        bounding_box
    }

    pub fn new_from_vec3_indices(points: &[Vec3], indices: &[usize]) -> Self{
        if indices.is_empty() || points.is_empty(){
            return Self{minimums: Vec3::zeros(), maximums: Vec3::zeros()};
        }
        let mut bounding_box = Self{ minimums: points[indices[0]], maximums: points[indices[0]] };
        for index in indices{
            bounding_box.grow_with_vec3(&points[*index]);
        }
        bounding_box
    }
 
    pub fn new_from_triangles(triangles: &[TriangleHitParser]) -> Self{
        if triangles.is_empty(){
            return Self{minimums: Vec3::zeros(), maximums: Vec3::zeros()};
        }
        let mut bounding_box = Self{ minimums: triangles[0].vertices[0], maximums: triangles[0].vertices[0] };
        for triangle in triangles{
            bounding_box.grow_with_triangle(triangle);
        }
        bounding_box
    }

    pub fn new_from_triangle_indices(triangles: &[TriangleHitParser], indices: &[usize]) -> Self{
        if indices.is_empty() || triangles.is_empty(){
            return Self{minimums: Vec3::zeros(), maximums: Vec3::zeros()};
        }
        let mut bounding_box = Self{ minimums: triangles[indices[0]].vertices[0], maximums: triangles[indices[0]].vertices[0] };
        for index in indices{
            bounding_box.grow_with_triangle(&triangles[*index]);
        }
        bounding_box
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
}
